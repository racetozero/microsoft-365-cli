//! `planner bucket *`, `planner plan *` and `planner roster *`.

use m365_core::prelude::*;
use m365_core::ut::{entra, planner};

/// `{ ...a, ...b }`.
pub fn merge(a: Value, b: &Value) -> Value {
    let mut a = match a {
        Value::Object(o) => o,
        _ => Obj::new(),
    };
    if let Some(b) = b.as_object() {
        for (k, v) in b {
            a.insert(k.clone(), v.clone());
        }
    }
    Value::Object(a)
}

async fn group_id(cx: &Cx) -> Result<String> {
    match cx.s("ownerGroupId") {
        Some(g) => Ok(g),
        None => entra::group::id_by_display_name(&cx.u("ownerGroupName")).await,
    }
}

/// `planId`, `planTitle` (with the owner group) or `rosterId`.
async fn plan_id(cx: &Cx) -> Result<String> {
    if let Some(p) = cx.s("planId") {
        return Ok(p);
    }
    if let Some(t) = cx.s("planTitle") {
        let g = group_id(cx).await?;
        return planner::plan_id_by_title(&t, &g).await;
    }
    planner::plan_id_by_roster_id(&cx.u("rosterId")).await
}

pub async fn bucket_add(cx: &mut Cx) -> Result<()> {
    let pid = plan_id(cx).await?;
    let r = Request::post(format!("{GRAPH}/v1.0/planner/buckets"))
        .graph_json()
        .json(obj! { "name": ?cx.val("name").cloned(), "planId": pid, "orderHint": ?cx.val("orderHint").cloned() })
        .data()
        .await?;
    cx.log(r)
}

pub async fn bucket_get(cx: &mut Cx) -> Result<()> {
    let b = match cx.s("id") {
        Some(i) => {
            Request::get(format!("{GRAPH}/v1.0/planner/buckets/{i}"))
                .graph_json()
                .data()
                .await?
        }
        None => {
            let pid = plan_id(cx).await?;
            planner::bucket_by_title(&cx.u("name"), &pid, "none").await?
        }
    };
    cx.log(b)
}

pub async fn bucket_list(cx: &mut Cx) -> Result<()> {
    let pid = plan_id(cx).await?;
    cx.log(Value::Array(
        odata::get_all_items(&format!("{GRAPH}/v1.0/planner/plans/{pid}/buckets")).await?,
    ))
}

async fn bucket_minimal(cx: &Cx) -> Result<Value> {
    match cx.s("id") {
        Some(i) => {
            Request::get(format!("{GRAPH}/v1.0/planner/buckets/{i}"))
                .accept("application/json;odata.metadata=minimal")
                .json_response()
                .data()
                .await
        }
        None => {
            let pid = plan_id(cx).await?;
            planner::bucket_by_title(&cx.u("name"), &pid, "minimal").await
        }
    }
}

pub async fn bucket_remove(cx: &mut Cx) -> Result<()> {
    let what = cx
        .s("id")
        .or_else(|| cx.s("name"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the bucket {what}?"),
    )? {
        return Ok(());
    }
    cx.verbose_log("Removing bucket...");
    let b = bucket_minimal(cx).await?;
    Request::delete(format!("{GRAPH}/v1.0/planner/buckets/{}", u(&b, "id")))
        .graph_json()
        .header("if-match", u(&b, "@odata.etag"))
        .send()
        .await?;
    Ok(())
}

pub async fn bucket_set(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Updating bucket...");
    let b = bucket_minimal(cx).await?;
    Request::patch(format!("{GRAPH}/v1.0/planner/buckets/{}", u(&b, "id")))
        .graph_json()
        .header("if-match", u(&b, "@odata.etag"))
        .json(obj! { "name": ?cx.val("newName").cloned(), "orderHint": ?cx.val("orderHint").cloned() })
        .send()
        .await?;
    Ok(())
}

// ---- plan ----------------------------------------------------------------------

async fn share_user_ids(cx: &Cx, sep: &str) -> Result<Vec<String>> {
    if let Some(ids) = cx.s("shareWithUserIds") {
        return Ok(ids.split(',').map(String::from).collect());
    }
    let users: Vec<String> = cx
        .u("shareWithUserNames")
        .split(',')
        .map(|o| o.trim().to_string())
        .collect();
    let mut upns = Vec::new();
    let mut ids = Vec::new();
    for user in &users {
        let r = Request::get(format!(
            "{GRAPH}/v1.0/users?$filter=userPrincipalName eq '{}'&$select=id,userPrincipalName",
            eqp(user)
        ))
        .header("content-type", "application/json")
        .json_response()
        .data()
        .await?;
        let first = arr(&r, "value").first().cloned();
        upns.push(
            first
                .as_ref()
                .and_then(|f| f.get("userPrincipalName"))
                .and_then(Value::as_str)
                .map(str::to_lowercase),
        );
        ids.push(
            first
                .as_ref()
                .map(|f| u(f, "id"))
                .unwrap_or_else(|| "undefined".into()),
        );
    }
    let invalid: Vec<&String> = users
        .iter()
        .filter(|u| {
            !upns
                .iter()
                .any(|p| p.as_deref() == Some(u.to_lowercase().as_str()))
        })
        .collect();
    if !invalid.is_empty() {
        return Err(Error::msg(format!(
            "Cannot proceed with planner plan creation. The following users provided are invalid{sep}: {}",
            invalid
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(",")
        )));
    }
    Ok(ids)
}

async fn shared_with(cx: &Cx, sep: &str) -> Result<Value> {
    let mut o = Obj::new();
    for id in share_user_ids(cx, sep).await? {
        o.insert(id, Value::Bool(true));
    }
    Ok(Value::Object(o))
}

async fn etag(url: String) -> Result<String> {
    let r = Request::get(url)
        .accept("application/json")
        .json_response()
        .data()
        .await?;
    Ok(u(&r, "@odata.etag"))
}

pub async fn plan_add(cx: &mut Cx) -> Result<()> {
    let r = async {
        let container = match cx.s("rosterId") {
            Some(r) => format!("https://graph.microsoft.com/v1.0/planner/rosters/{r}"),
            None => format!(
                "https://graph.microsoft.com/v1.0/groups/{}",
                group_id(cx).await?
            ),
        };
        let plan = Request::post(format!("{GRAPH}/v1.0/planner/plans"))
            .accept("application/json;odata.metadata=none")
            .json(
                obj! { "title": ?cx.val("title").cloned(), "container": obj! { "url": container } },
            )
            .json_response()
            .data()
            .await?;
        if cx.s("shareWithUserIds").is_none() && cx.s("shareWithUserNames").is_none() {
            return Ok(plan);
        }
        let pid = u(&plan, "id");
        // Promise.all: both requests are sent even when one fails
        let tag = etag(format!("{GRAPH}/v1.0/planner/plans/{pid}/details")).await;
        let sw = shared_with(cx, " ").await?;
        let tag = tag?;
        let d = Request::patch(format!("{GRAPH}/v1.0/planner/plans/{pid}/details"))
            .accept("application/json;odata.metadata=none")
            .header("If-Match", tag)
            .header("Prefer", "return=representation")
            .json(obj! { "sharedWith": sw })
            .json_response()
            .data()
            .await?;
        Ok::<Value, Error>(merge(plan, &d))
    }
    .await;
    match r {
        Ok(v) => cx.log(v),
        Err(e)
            if cx.s("rosterId").is_some()
                && e.body()
                    .and_then(|b| b.get("error"))
                    .and_then(|e| e.get("message"))
                    .and_then(Value::as_str)
                    == Some(
                        "You do not have the required permissions to access this item, or the item may not exist.",
                    ) =>
        {
            Err(Error::msg("You can only add 1 plan to a Roster"))
        }
        Err(e) => Err(e),
    }
}

async fn plan_details(plan: Value) -> Result<Value> {
    let d = Request::get(format!(
        "{GRAPH}/v1.0/planner/plans/{}/details",
        u(&plan, "id")
    ))
    .graph_json()
    .header("Prefer", "return=representation")
    .data()
    .await?;
    Ok(merge(plan, &d))
}

pub async fn plan_get(cx: &mut Cx) -> Result<()> {
    let plan = if let Some(i) = cx.s("id") {
        planner::plan_by_id(&i, "none").await?
    } else if let Some(r) = cx.s("rosterId") {
        planner::plan_by_roster_id(&r, "none").await?
    } else {
        let g = group_id(cx).await?;
        planner::plan_by_title(&cx.u("title"), &g, "none").await?
    };
    cx.log(plan_details(plan).await?)
}

pub async fn plan_list(cx: &mut Cx) -> Result<()> {
    let plans = if cx.s("ownerGroupId").is_some() || cx.s("ownerGroupName").is_some() {
        let g = group_id(cx).await?;
        planner::plans_by_group_id(&g, "none").await?
    } else {
        vec![planner::plan_by_roster_id(&cx.u("rosterId"), "none").await?]
    };
    cx.log(Value::Array(plans))
}

pub async fn plan_remove(cx: &mut Cx) -> Result<()> {
    let what = cx
        .s("id")
        .or_else(|| cx.s("title"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the plan {what}?"),
    )? {
        return Ok(());
    }
    let plan = match cx.s("id") {
        Some(i) => planner::plan_by_id(&i, "minimal").await?,
        None => {
            let g = group_id(cx).await?;
            planner::plan_by_title(&cx.u("title"), &g, "minimal").await?
        }
    };
    cx.verbose_log(&format!("Removing plan '{}' ...", u(&plan, "title")));
    Request::delete(format!("{GRAPH}/v1.0/planner/plans/{}", u(&plan, "id")))
        .graph_json()
        .header("if-match", u(&plan, "@odata.etag"))
        .send()
        .await?;
    Ok(())
}

pub fn plan_set_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    Ok(o.keys().filter(|k| k.contains("category")).all(|k| {
        k.strip_prefix("category")
            .and_then(|n| n.parse::<u8>().ok())
            .is_some_and(|n| (1..=25).contains(&n) && k == &format!("category{n}"))
    }))
}

pub async fn plan_set(cx: &mut Cx) -> Result<()> {
    let pid = if let Some(i) = cx.s("id") {
        i
    } else if let Some(r) = cx.s("rosterId") {
        planner::plan_id_by_roster_id(&r).await?
    } else {
        let g = group_id(cx).await?;
        planner::plan_id_by_title(&cx.u("title"), &g).await?
    };
    if let Some(t) = cx.s("newTitle") {
        let tag = etag(format!("{GRAPH}/v1.0/planner/plans/{pid}")).await?;
        Request::patch(format!("{GRAPH}/v1.0/planner/plans/{pid}"))
            .graph_json()
            .header("If-Match", tag)
            .header("Prefer", "return=representation")
            .json(obj! { "title": t })
            .send()
            .await?;
    }
    let plan = planner::plan_by_id(&pid, "none").await?;
    let mut cats = Obj::new();
    for (k, v) in &cx.opts {
        if k.contains("category") {
            cats.insert(k.clone(), v.clone());
        }
    }
    let share = cx.s("shareWithUserIds").is_some() || cx.s("shareWithUserNames").is_some();
    if !share && cats.is_empty() {
        return cx.log(plan_details(plan).await?);
    }
    let mut body = Obj::new();
    if share {
        body.insert("sharedWith".into(), shared_with(cx, "").await?);
    }
    if !cats.is_empty() {
        body.insert("categoryDescriptions".into(), Value::Object(cats));
    }
    let tag = etag(format!("{GRAPH}/v1.0/planner/plans/{pid}/details")).await?;
    let d = Request::patch(format!("{GRAPH}/v1.0/planner/plans/{pid}/details"))
        .accept("application/json;odata.metadata=none")
        .header("If-Match", tag)
        .header("Prefer", "return=representation")
        .json(Value::Object(body))
        .json_response()
        .data()
        .await?;
    cx.log(merge(plan, &d))
}

// ---- roster --------------------------------------------------------------------

pub async fn roster_add(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Creating a new Microsoft Planner Roster");
    let r = Request::post(format!("{GRAPH}/beta/planner/rosters"))
        .graph_json()
        .json(obj! {})
        .data()
        .await?;
    cx.log(r)
}

pub async fn roster_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving information about Microsoft Planner Roster with id {}",
        cx.u("id")
    ));
    let r = Request::get(format!("{GRAPH}/beta/planner/rosters/{}", cx.u("id")))
        .graph_json()
        .data()
        .await?;
    cx.log(r)
}

async fn roster_user_id(cx: &Cx) -> Result<String> {
    match cx.s("userId") {
        Some(i) => Ok(i),
        None => entra::user::id_by_upn(&cx.u("userName")).await,
    }
}

pub async fn roster_member_add(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Adding a user to a Microsoft Planner Roster");
    cx.verbose_log("Getting the user ID");
    let uid = roster_user_id(cx).await?;
    let r = Request::post(format!(
        "{GRAPH}/beta/planner/rosters/{}/members",
        cx.u("rosterId")
    ))
    .graph_json()
    .json(obj! { "userId": uid })
    .data()
    .await?;
    cx.log(r)
}

fn name_or_id(cx: &Cx) -> String {
    cx.s("userName")
        .or_else(|| cx.s("userId"))
        .unwrap_or_else(|| "undefined".into())
}

pub async fn roster_member_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving member {} from the Microsoft Planner Roster",
        name_or_id(cx)
    ));
    let uid = roster_user_id(cx).await?;
    let r = Request::get(format!(
        "{GRAPH}/beta/planner/rosters/{}/members/{uid}",
        cx.u("rosterId")
    ))
    .graph_json()
    .data()
    .await?;
    cx.log(r)
}

pub async fn roster_member_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving members of the specified Microsoft Planner Roster");
    cx.log(Value::Array(
        odata::get_all_items(&format!(
            "{GRAPH}/beta/planner/rosters/{}/members",
            cx.u("rosterId")
        ))
        .await?,
    ))
}

pub async fn roster_member_remove(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Removing member {} from the Microsoft Planner Roster",
        name_or_id(cx)
    ));
    let who = cx
        .s("userId")
        .or_else(|| cx.s("userName"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove member '{who}'?"),
    )? {
        return Ok(());
    }
    let roster = cx.u("rosterId");
    if !cx.bool("force") {
        let members = odata::get_all_items(&format!(
            "{GRAPH}/beta/planner/rosters/{roster}/members?$select=Id"
        ))
        .await?;
        if members.len() == 1
            && !cx.confirm("You are about to remove the last member of this Roster. When this happens, the Roster and all its contents will be deleted within 30 days. Are you sure you want to proceed?")?
        {
            return Ok(());
        }
    }
    let uid = roster_user_id(cx).await?;
    Request::delete(format!(
        "{GRAPH}/beta/planner/rosters/{roster}/members/{uid}"
    ))
    .graph_json()
    .send()
    .await?;
    Ok(())
}

pub async fn roster_plan_list(cx: &mut Cx) -> Result<()> {
    let app =
        auth::jwt::is_app_only(&auth::default_access_token().unwrap_or_default()) == Some(true);
    let user = cx.s("userId").or_else(|| cx.s("userName"));
    if app && user.is_none() {
        return Err(Error::msg(
            "Specify at least 'userId' or 'userName' when using application permissions.",
        ));
    } else if !app && user.is_some() {
        return Err(Error::msg(
            "The options 'userId' or 'userName' cannot be used when obtaining Microsoft Planner Roster plans using delegated permissions.",
        ));
    }
    cx.verbose_log(&format!(
        "Retrieving Microsoft Planner Roster plans for user: {}.",
        user.clone().unwrap_or_else(|| "current user".into())
    ));
    let who = user
        .map(|u| format!("users/{u}"))
        .unwrap_or_else(|| "me".into());
    cx.log(Value::Array(
        odata::get_all_items(&format!("{GRAPH}/beta/{who}/planner/rosterPlans")).await?,
    ))
}

pub async fn roster_remove(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove roster {id}?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!("Removing roster {id}"));
    Request::delete(format!("{GRAPH}/beta/planner/rosters/{id}"))
        .graph_json()
        .send()
        .await?;
    Ok(())
}

// ---- task ----------------------------------------------------------------------

const PRIORITIES: [&str; 4] = ["Urgent", "Important", "Medium", "Low"];
const CATEGORIES: [&str; 6] = [
    "category1",
    "category2",
    "category3",
    "category4",
    "category5",
    "category6",
];

/// `taskPriority.getPriorityValue`: names map to numbers, other strings pass through.
fn priority_value(p: &Value) -> Value {
    match p.as_str().map(str::to_lowercase).as_deref() {
        Some("urgent") => Value::from(1),
        Some("important") => Value::from(3),
        Some("medium") => Value::from(5),
        Some("low") => Value::from(9),
        _ => p.clone(),
    }
}

/// Field hooks shared by `task add` and `task set` (`set` has no previewType,
/// so its priority check is index 4 as well as 5 in `add`).
fn task_field(i: u16, v: &Value, add: bool) -> FieldCheck {
    let s = to_js_string(v);
    match (i, add) {
        (1, _) => match m365_core::validation::is_valid_guid_array(&s) {
            m365_core::validation::Valid::Msg(m) => Err(Some(format!(
                "The following GUIDs are invalid for the option 'assignedToUserIds': {m}."
            ))),
            _ => Ok(()),
        },
        (2, _) => match m365_core::validation::is_valid_user_principal_name_array(&s) {
            m365_core::validation::Valid::Msg(m) => Err(Some(format!(
                "The following user principal names are invalid for the option 'assignedToUserNames': {m}."
            ))),
            _ => Ok(()),
        },
        (3, _) => check(
            s.split(',')
                .all(|c| CATEGORIES.contains(&c.to_lowercase().trim())),
        ),
        (4, true) => check(
            [
                "automatic",
                "nopreview",
                "checklist",
                "description",
                "reference",
            ]
            .contains(&s.to_lowercase().as_str()),
        ),
        _ => {
            let n = json::js_number(&s);
            if !n.is_nan() {
                check((0.0..=10.0).contains(&n) && n.fract() == 0.0)
            } else {
                check(
                    PRIORITIES
                        .iter()
                        .any(|p| p.to_lowercase() == s.to_lowercase()),
                )
            }
        }
    }
}

pub fn task_add_field(i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    task_field(i, v, true)
}

pub fn task_set_field(i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    task_field(i, v, false)
}

async fn task_user_ids(
    cx: &Cx,
    action: &str,
    trim_ids: bool,
    accept_header: bool,
) -> Result<Vec<String>> {
    if let Some(ids) = cx.s("assignedToUserIds") {
        return Ok(ids
            .split(',')
            .map(|o| {
                if trim_ids {
                    o.trim().to_string()
                } else {
                    o.to_string()
                }
            })
            .collect());
    }
    let users: Vec<String> = cx
        .u("assignedToUserNames")
        .split(',')
        .map(|o| o.trim().to_string())
        .collect();
    let mut upns = Vec::new();
    let mut ids = Vec::new();
    for user in &users {
        let req = Request::get(format!(
            "{GRAPH}/v1.0/users?$filter=userPrincipalName eq '{}'&$select=id,userPrincipalName",
            eqp(user)
        ))
        .json_response();
        let req = if accept_header {
            req.header("accept ", "application/json;odata.metadata=none")
        } else {
            req.header("content-type", "application/json")
        };
        let r = req.data().await?;
        let first = arr(&r, "value").first().cloned();
        upns.push(
            first
                .as_ref()
                .and_then(|f| f.get("userPrincipalName"))
                .and_then(Value::as_str)
                .map(str::to_lowercase),
        );
        ids.push(
            first
                .as_ref()
                .map(|f| u(f, "id"))
                .unwrap_or_else(|| "undefined".into()),
        );
    }
    let invalid: Vec<&str> = users
        .iter()
        .filter(|u| {
            !upns
                .iter()
                .any(|p| p.as_deref() == Some(u.to_lowercase().as_str()))
        })
        .map(String::as_str)
        .collect();
    if !invalid.is_empty() {
        return Err(Error::msg(format!(
            "Cannot proceed with planner task {action}. The following users provided are invalid : {}",
            invalid.join(",")
        )));
    }
    Ok(ids)
}

async fn assignments(cx: &Cx, action: &str, set: bool) -> Result<Obj> {
    let mut a = Obj::new();
    if cx.s("assignedToUserIds").is_none() && cx.s("assignedToUserNames").is_none() {
        return Ok(a);
    }
    for id in task_user_ids(cx, action, set, set).await? {
        a.insert(
            id,
            obj! { "@odata.type": "#microsoft.graph.plannerAssignment", "orderHint": " !" },
        );
    }
    Ok(a)
}

fn applied_categories(cx: &Cx) -> Obj {
    let mut c = Obj::new();
    if let Some(a) = cx.s("appliedCategories") {
        for x in a.to_lowercase().split(',') {
            c.insert(x.trim().to_string(), Value::Bool(true));
        }
    }
    c
}

/// `planId`, `rosterId` or `planTitle` with the owner group (task commands
/// check `rosterId` before the title).
async fn task_plan_id(cx: &Cx) -> Result<String> {
    if let Some(p) = cx.s("planId") {
        return Ok(p);
    }
    if let Some(r) = cx.s("rosterId") {
        return planner::plan_id_by_roster_id(&r).await;
    }
    let g = group_id(cx).await?;
    planner::plan_id_by_title(&cx.u("planTitle"), &g).await
}

async fn task_bucket_id(cx: &Cx) -> Result<String> {
    match cx.s("bucketId") {
        Some(b) => Ok(b),
        None => {
            let pid = task_plan_id(cx).await?;
            planner::bucket_id_by_title(&cx.u("bucketName"), &pid).await
        }
    }
}

async fn task_details_etag(task_id: &str) -> Result<String> {
    etag(format!(
        "{GRAPH}/v1.0/planner/tasks/{}/details",
        eqp(task_id)
    ))
    .await
}

async fn patch_details(task_id: &str, tag: String, body: Value) -> Result<Value> {
    Request::patch(format!("{GRAPH}/v1.0/planner/tasks/{task_id}/details"))
        .accept("application/json;odata.metadata=none")
        .header("If-Match", tag)
        .header("Prefer", "return=representation")
        .json(body)
        .json_response()
        .data()
        .await
}

pub async fn task_add(cx: &mut Cx) -> Result<()> {
    let pid = task_plan_id(cx).await?;
    let bid = match cx.s("bucketId") {
        Some(b) => b,
        None => planner::bucket_id_by_title(&cx.u("bucketName"), &pid).await?,
    };
    let a = assignments(cx, "creation", false).await?;
    let data = obj! {
        "planId": pid,
        "bucketId": bid,
        "title": ?cx.val("title").cloned(),
        "startDateTime": ?cx.val("startDateTime").cloned(),
        "dueDateTime": ?cx.val("dueDateTime").cloned(),
        "percentComplete": ?cx.s("percentComplete").map(|p| json::num_value(json::js_number(&p))),
        "assignments": Value::Object(a),
        "orderHint": ?cx.val("orderHint").cloned(),
        "assigneePriority": ?cx.val("assigneePriority").cloned(),
        "appliedCategories": Value::Object(applied_categories(cx)),
        "priority": ?cx.val("priority").map(priority_value),
    };
    let task = Request::post(format!("{GRAPH}/v1.0/planner/tasks"))
        .graph_json()
        .json(data)
        .data()
        .await?;
    let mut result = if cx.s("description").is_none() && cx.s("previewType").is_none() {
        task
    } else {
        let tid = u(&task, "id");
        let tag = task_details_etag(&tid).await?;
        let d = patch_details(&tid, tag, obj! { "description": ?cx.val("description").cloned(), "previewType": ?cx.val("previewType").cloned() }).await?;
        merge(task, &d)
    };
    if result.get("description").is_some_and(truthy) {
        result
            .as_object_mut()
            .map(|o| o.insert("hasDescription".into(), Value::Bool(true)));
    }
    cx.log(result)
}

async fn task_details_raw(task_id: &str, accept: &str) -> Result<Value> {
    Request::get(format!(
        "{GRAPH}/v1.0/planner/tasks/{}/details?$select=checklist",
        eqp(task_id)
    ))
    .accept(accept)
    .json_response()
    .data()
    .await
}

fn checklist_output(cx: &Cx, checklist: Option<Value>) -> Result<()> {
    let Some(checklist) = checklist else {
        return if cx.str("output").as_deref() == Some("text") {
            Err(Error::js("Cannot convert undefined or null to object"))
        } else {
            cx.log_undefined()
        };
    };
    if cx.str("output").as_deref() != Some("text") {
        return cx.log(checklist);
    }
    let out: Vec<Value> = checklist
        .as_object()
        .map(|o| {
            o.iter()
                .map(|(k, v)| merge(obj! { "id": k.as_str() }, v))
                .collect()
        })
        .unwrap_or_default();
    cx.log(Value::Array(out))
}

pub async fn task_checklistitem_add(cx: &mut Cx) -> Result<()> {
    let tid = cx.u("taskId");
    let r = Request::get(format!("{GRAPH}/v1.0/planner/tasks/{}/details", eqp(&tid)))
        .accept("application/json;odata.metadata=minimal")
        .json_response()
        .data()
        .await?;
    let mut item = Obj::new();
    item.insert(
        util::new_guid(),
        obj! { "@odata.type": "microsoft.graph.plannerChecklistItem", "title": ?cx.val("title").cloned(), "isChecked": cx.opt_bool("isChecked").unwrap_or(false) },
    );
    let res = Request::patch(format!("{GRAPH}/v1.0/planner/tasks/{}/details", eqp(&tid)))
        .accept("application/json;odata.metadata=none")
        .header("prefer", "return=representation")
        .header("if-match", u(&r, "@odata.etag"))
        .json(obj! { "checklist": Value::Object(item) })
        .json_response()
        .data()
        .await?;
    checklist_output(cx, res.get("checklist").cloned())
}

pub async fn task_checklistitem_list(cx: &mut Cx) -> Result<()> {
    let r = task_details_raw(&cx.u("taskId"), "application/json;odata.metadata=none").await?;
    checklist_output(cx, r.get("checklist").cloned())
}

pub async fn task_checklistitem_remove(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove the checklist item with id {id} from the planner task?"
        ),
    )? {
        return Ok(());
    }
    let tid = cx.u("taskId");
    let task = task_details_raw(&tid, "application/json;odata.metadata=minimal").await?;
    if !task
        .get("checklist")
        .and_then(|c| c.get(&id))
        .is_some_and(truthy)
    {
        return Err(Error::msg(format!(
            "The specified checklist item with id {id} does not exist"
        )));
    }
    let mut cl = Obj::new();
    cl.insert(id, Value::Null);
    patch_details(
        &tid,
        u(&task, "@odata.etag"),
        obj! { "checklist": Value::Object(cl) },
    )
    .await?;
    Ok(())
}

async fn task_id_by_title(cx: &Cx) -> Result<String> {
    if let Some(i) = cx.s("id") {
        return Ok(i);
    }
    let bid = task_bucket_id(cx).await?;
    let r = Request::get(format!(
        "{GRAPH}/v1.0/planner/buckets/{bid}/tasks?$select=id,title"
    ))
    .graph_json()
    .data()
    .await?;
    let title = cx.u("title");
    let tasks: Vec<Value> = arr(&r, "value")
        .iter()
        .filter(|t| {
            t.get("title")
                .and_then(Value::as_str)
                .map(str::to_lowercase)
                == Some(title.to_lowercase())
        })
        .cloned()
        .collect();
    match tasks.len() {
        0 => Err(Error::msg(format!(
            "The specified task {title} does not exist"
        ))),
        1 => Ok(u(&tasks[0], "id")),
        _ => Ok(u(
            &cx.handle_multiple_results(
                &format!("Multiple tasks with title '{title}' found."),
                &util::array_to_hash_table("id", &tasks),
            )?,
            "id",
        )),
    }
}

pub async fn task_get(cx: &mut Cx) -> Result<()> {
    let tid = task_id_by_title(cx).await?;
    let task = Request::get(format!("{GRAPH}/v1.0/planner/tasks/{}", eqp(&tid)))
        .graph_json()
        .data()
        .await?;
    let d = Request::get(format!(
        "{GRAPH}/v1.0/planner/tasks/{}/details",
        u(&task, "id")
    ))
    .graph_json()
    .header("Prefer", "return=representation")
    .data()
    .await?;
    cx.log(merge(task, &d))
}

pub async fn task_list(cx: &mut Cx) -> Result<()> {
    let (v1, beta) = if cx.s("bucketId").is_some() || cx.s("bucketName").is_some() {
        let bid = match cx.s("bucketId") {
            Some(b) => eqp(&b),
            None => {
                let pid = list_plan_id(cx).await?;
                planner::bucket_id_by_title(&cx.u("bucketName"), &pid).await?
            }
        };
        (
            format!("{GRAPH}/v1.0/planner/buckets/{bid}/tasks"),
            format!("{GRAPH}/beta/planner/buckets/{bid}/tasks"),
        )
    } else if cx.s("planId").is_some() || cx.s("planTitle").is_some() {
        let pid = list_plan_id(cx).await?;
        (
            format!("{GRAPH}/v1.0/planner/plans/{pid}/tasks"),
            format!("{GRAPH}/beta/planner/plans/{pid}/tasks"),
        )
    } else {
        (
            format!("{GRAPH}/v1.0/me/planner/tasks"),
            format!("{GRAPH}/beta/me/planner/tasks"),
        )
    };
    let mut items = odata::get_all_items(&v1).await?;
    let beta = odata::get_all_items(&beta).await?;
    for t in items.iter_mut() {
        if let Some(b) = beta.iter().find(|b| b.get("id") == t.get("id"))
            && let (Some(p), Some(o)) = (b.get("priority").cloned(), t.as_object_mut())
        {
            o.insert("priority".into(), p);
        }
    }
    cx.log(Value::Array(items))
}

/// `task list` encodes ids it was given.
async fn list_plan_id(cx: &Cx) -> Result<String> {
    if let Some(p) = cx.s("planId") {
        return Ok(eqp(&p));
    }
    if let Some(r) = cx.s("rosterId") {
        return planner::plan_id_by_roster_id(&r).await;
    }
    let g = match cx.s("ownerGroupId") {
        Some(g) => eqp(&g),
        None => entra::group::id_by_display_name(&cx.u("ownerGroupName")).await?,
    };
    planner::plan_id_by_title(&cx.u("planTitle"), &g).await
}

pub async fn task_reference_add(cx: &mut Cx) -> Result<()> {
    let tid = cx.u("taskId");
    let tag = task_details_etag(&tid).await?;
    let mut refs = Obj::new();
    refs.insert(
        util::open_types_encoder(&cx.u("url")),
        obj! { "@odata.type": "microsoft.graph.plannerExternalReference", "previewPriority": " !", "alias": ?cx.s("alias"), "type": ?cx.s("type") },
    );
    let r = patch_details(&eqp(&tid), tag, obj! { "references": Value::Object(refs) }).await?;
    match r.get("references") {
        Some(v) => cx.log(v.clone()),
        None => cx.log_undefined(),
    }
}

pub async fn task_reference_list(cx: &mut Cx) -> Result<()> {
    let r = Request::get(format!(
        "{GRAPH}/v1.0/planner/tasks/{}/details?$select=references",
        eqp(&cx.u("taskId"))
    ))
    .graph_json()
    .data()
    .await?;
    match r.get("references") {
        Some(v) => cx.log(v.clone()),
        None => cx.log_undefined(),
    }
}

pub async fn task_reference_remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        "Are you sure you want to remove the reference from the Planner task?",
    )? {
        return Ok(());
    }
    let tid = cx.u("taskId");
    let d = Request::get(format!("{GRAPH}/v1.0/planner/tasks/{}/details", eqp(&tid)))
        .accept("application/json")
        .json_response()
        .data()
        .await?;
    let mut url = cx.u("url");
    if let Some(alias) = cx.s("alias") {
        let urls: Vec<String> = d
            .get("references")
            .and_then(Value::as_object)
            .map(|o| {
                o.iter()
                    .filter(|(_, v)| {
                        v.get("alias")
                            .and_then(Value::as_str)
                            .map(str::to_lowercase)
                            == Some(alias.to_lowercase())
                    })
                    .map(|(k, _)| util::dec(k))
                    .collect()
            })
            .unwrap_or_default();
        if urls.is_empty() {
            return Err(Error::msg(format!(
                "The specified reference with alias {alias} does not exist"
            )));
        }
        if urls.len() > 1 {
            return Err(Error::msg(format!(
                "Multiple references with alias {alias} found. Pass one of the following urls within the \"--url\" option : {}",
                urls.join(",")
            )));
        }
        url = urls[0].clone();
    }
    let mut refs = Obj::new();
    refs.insert(util::open_types_encoder(&url), Value::Null);
    patch_details(
        &tid,
        u(&d, "@odata.etag"),
        obj! { "references": Value::Object(refs) },
    )
    .await?;
    Ok(())
}

pub async fn task_remove(cx: &mut Cx) -> Result<()> {
    let what = cx
        .s("id")
        .or_else(|| cx.s("title"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the task {what}?"),
    )? {
        return Ok(());
    }
    let task = match cx.s("id") {
        Some(i) => {
            Request::get(format!("{GRAPH}/v1.0/planner/tasks/{i}"))
                .accept("application/json")
                .json_response()
                .data()
                .await?
        }
        None => {
            let bid = task_bucket_id(cx).await?;
            let tasks = odata::get_all_items_meta(
                &format!("{GRAPH}/v1.0/planner/buckets/{bid}/tasks?$select=title,id"),
                "minimal",
            )
            .await?;
            let title = cx.u("title");
            let f: Vec<Value> = tasks
                .into_iter()
                .filter(|t| s(t, "title").to_lowercase() == title.to_lowercase())
                .collect();
            match f.len() {
                0 => {
                    return Err(Error::msg(format!(
                        "The specified task {title} does not exist"
                    )));
                }
                1 => f.into_iter().next().unwrap_or(Value::Null),
                _ => cx.handle_multiple_results(
                    &format!("Multiple tasks with title '{title}' found."),
                    &util::array_to_hash_table("id", &f),
                )?,
            }
        }
    };
    cx.verbose_log(&format!("Removing task '{}' ...", u(&task, "title")));
    Request::delete(format!("{GRAPH}/v1.0/planner/tasks/{}", u(&task, "id")))
        .graph_json()
        .header("if-match", u(&task, "@odata.etag"))
        .send()
        .await?;
    Ok(())
}

pub async fn task_set(cx: &mut Cx) -> Result<()> {
    let bid = if cx.s("bucketId").is_some() || cx.s("bucketName").is_some() {
        Some(task_bucket_id(cx).await?)
    } else {
        None
    };
    let a = assignments(cx, "update", true).await?;
    let id = cx.u("id");
    let tag = etag(format!("{GRAPH}/v1.0/planner/tasks/{}", eqp(&id))).await?;
    let cats = applied_categories(cx);
    let mut body = Obj::new();
    let mut put = |k: &str, v: Option<Value>| {
        if let Some(v) = v {
            body.insert(k.into(), v);
        }
    };
    put("title", cx.s("title").map(Value::from));
    put("bucketId", bid.filter(|b| !b.is_empty()).map(Value::from));
    put("startDateTime", cx.s("startDateTime").map(Value::from));
    put("dueDateTime", cx.s("dueDateTime").map(Value::from));
    put(
        "percentComplete",
        cx.s("percentComplete")
            .map(|p| json::num_value(json::js_number(&p))),
    );
    put("assignments", (!a.is_empty()).then_some(Value::Object(a)));
    put(
        "assigneePriority",
        cx.s("assigneePriority").map(Value::from),
    );
    put(
        "appliedCategories",
        (!cats.is_empty()).then_some(Value::Object(cats)),
    );
    put("orderHint", cx.s("orderHint").map(Value::from));
    put("priority", cx.val("priority").map(priority_value));
    let task = Request::patch(format!("{GRAPH}/v1.0/planner/tasks/{id}"))
        .accept("application/json;odata.metadata=none")
        .header("If-Match", tag)
        .header("Prefer", "return=representation")
        .json(Value::Object(body))
        .json_response()
        .data()
        .await?;
    let result = match cx.s("description") {
        None => task,
        Some(desc) => {
            let tid = u(&task, "id");
            let tag = task_details_etag(&tid).await?;
            let d = patch_details(&tid, tag, obj! { "description": desc }).await?;
            merge(task, &d)
        }
    };
    cx.log(result)
}

// ---- tenant --------------------------------------------------------------------

pub async fn tenant_settings_list(cx: &mut Cx) -> Result<()> {
    let r = Request::get("https://tasks.office.com/taskAPI/tenantAdminSettings/Settings")
        .graph_json()
        .data()
        .await?;
    cx.log(r)
}

pub async fn tenant_settings_set(cx: &mut Cx) -> Result<()> {
    let keys = [
        "isPlannerAllowed",
        "allowCalendarSharing",
        "allowTenantMoveWithDataLoss",
        "allowTenantMoveWithDataMigration",
        "allowRosterCreation",
        "allowPlannerMobilePushNotifications",
    ];
    let mut data = Obj::new();
    for k in keys {
        if let Some(v) = cx.val(k) {
            data.insert(k.into(), v.clone());
        }
    }
    let r = Request::patch("https://tasks.office.com/taskAPI/tenantAdminSettings/Settings")
        .graph_json()
        .header("prefer", "return=representation")
        .json(Value::Object(data))
        .data()
        .await?;
    cx.log(r)
}
