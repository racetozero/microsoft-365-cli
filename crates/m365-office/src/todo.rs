//! `todo list *` and `todo task *`.

use m365_core::prelude::*;

const LISTS: &str = "https://graph.microsoft.com/v1.0/me/todo/lists";
const STATUSES: [&str; 5] = [
    "notStarted",
    "inProgress",
    "completed",
    "waitingOnOthers",
    "deferred",
];

fn list_by_name_url(name: &str) -> String {
    format!("{LISTS}?$filter=displayName eq '{}'", eqp(name))
}

pub async fn list_add(cx: &mut Cx) -> Result<()> {
    let r = Request::post(LISTS)
        .graph_json()
        .header("content-type", "application/json")
        .json(obj! { "displayName": ?cx.val("name").cloned() })
        .data()
        .await?;
    cx.log(r)
}

pub async fn list_get(cx: &mut Cx) -> Result<()> {
    let item = match cx.s("id") {
        Some(i) => {
            Request::get(format!("{LISTS}/{i}"))
                .graph_json()
                .data()
                .await?
        }
        None => {
            let r = Request::get(list_by_name_url(&cx.u("name")))
                .graph_json()
                .data()
                .await?;
            match arr(&r, "value").first() {
                Some(v) => v.clone(),
                None => {
                    return Err(Error::msg(format!(
                        "The specified list '{}' does not exist.",
                        cx.u("name")
                    )));
                }
            }
        }
    };
    cx.log(item)
}

pub async fn list_list(cx: &mut Cx) -> Result<()> {
    cx.log(Value::Array(odata::get_all_items(LISTS).await?))
}

/// The id of the only list with the name, if exactly one matches.
async fn single_list_id(name: &str) -> Result<Option<String>> {
    let r = Request::get(list_by_name_url(name))
        .graph_json()
        .data()
        .await?;
    let v = arr(&r, "value");
    Ok((v.len() == 1).then(|| u(&v[0], "id")))
}

pub async fn list_remove(cx: &mut Cx) -> Result<()> {
    let what = cx
        .str("id")
        .or_else(|| cx.str("name"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the task list {what}?"),
    )? {
        return Ok(());
    }
    let id = match cx.s("id") {
        Some(i) => Some(i),
        None => single_list_id(&cx.u("name")).await?,
    };
    let Some(id) = id.filter(|i| !i.is_empty()) else {
        return Err(Error::msg(format!(
            "The list {} cannot be found",
            cx.u("name")
        )));
    };
    Request::delete(format!("{LISTS}/{id}"))
        .graph_json()
        .send()
        .await?;
    Ok(())
}

pub async fn list_set(cx: &mut Cx) -> Result<()> {
    let id = match cx.s("id") {
        Some(i) => Some(i),
        None => single_list_id(&cx.u("name")).await?,
    };
    let Some(id) = id.filter(|i| !i.is_empty()) else {
        return Err(Error::msg(format!(
            "The list {} cannot be found",
            cx.u("name")
        )));
    };
    Request::patch(format!("{LISTS}/{id}"))
        .graph_json()
        .header("content-type", "application/json")
        .json(obj! { "displayName": ?cx.val("newName").cloned() })
        .send()
        .await?;
    Ok(())
}

// ---- task ----------------------------------------------------------------------

fn s_opt(o: &Obj, k: &str) -> Option<String> {
    o.get(k).filter(|v| truthy(v)).map(to_js_string)
}

fn common_validate(o: &Obj) -> std::result::Result<(), String> {
    if let Some(b) = s_opt(o, "bodyContentType")
        && !["text", "html"].contains(&b.to_lowercase().as_str())
    {
        return Err(format!(
            "'{b}' is not a valid value for the bodyContentType option. Allowed values are text|html"
        ));
    }
    if let Some(i) = s_opt(o, "importance")
        && !["low", "normal", "high"].contains(&i.to_lowercase().as_str())
    {
        return Err(format!(
            "'{i}' is not a valid value for the importance option. Allowed values are low|normal|high"
        ));
    }
    for (k, suffix) in [
        ("dueDateTime", "is not a valid ISO date string"),
        ("reminderDateTime", "is not a valid ISO date string"),
        ("completedDateTime", "is not a valid datetime."),
        ("startDateTime", "is not a valid datetime."),
    ] {
        if let Some(d) = s_opt(o, k)
            && !m365_core::validation::is_valid_iso_date_time(&d)
        {
            return Err(format!("'{d}' {suffix}"));
        }
    }
    Ok(())
}

pub fn task_add_validate(o: &Obj) -> std::result::Result<(), String> {
    common_validate(o)?;
    let status = s_opt(o, "status");
    if let Some(st) = &status
        && !STATUSES
            .iter()
            .any(|x| x.to_lowercase() == st.to_lowercase())
    {
        return Err(format!(
            "{st} is not a valid value for status. Valid values are {}",
            STATUSES.join(", ")
        ));
    }
    if s_opt(o, "completedDateTime").is_some()
        && status.map(|s| s.to_lowercase()).as_deref() != Some("completed")
    {
        return Err("The completedDateTime option can only be used when the status option is set to completed".into());
    }
    Ok(())
}

pub fn task_set_validate(o: &Obj) -> std::result::Result<(), String> {
    if let Some(st) = s_opt(o, "status")
        && !STATUSES.contains(&st.as_str())
    {
        return Err(format!(
            "{st} is not a valid value. Allowed values are notStarted|inProgress|completed|waitingOnOthers|deferred"
        ));
    }
    common_validate(o)
}

async fn todo_list_id(cx: &Cx) -> Result<String> {
    if let Some(i) = cx.str("listId").filter(|i| !i.is_empty()) {
        return Ok(i);
    }
    let r = Request::get(list_by_name_url(&cx.u("listName")))
        .graph_json()
        .data()
        .await?;
    match arr(&r, "value").first() {
        Some(l) => Ok(u(l, "id")),
        None => Err(Error::msg("The specified task list does not exist")),
    }
}

fn dttz(v: Option<String>) -> Option<Value> {
    v.map(|d| obj! { "dateTime": d, "timeZone": "Etc/GMT" })
}

pub async fn task_add(cx: &mut Cx) -> Result<()> {
    let lid = todo_list_id(cx).await?;
    let status = cx.s("status").and_then(|st| {
        STATUSES
            .iter()
            .find(|x| x.to_lowercase() == st.to_lowercase())
            .map(|x| x.to_string())
    });
    let data = obj! {
        "title": ?cx.val("title").cloned(),
        "body": obj! {
            "content": ?cx.val("bodyContent").cloned(),
            "contentType": cx.s("bodyContentType").map(|b| b.to_lowercase()).filter(|b| !b.is_empty()).unwrap_or_else(|| "text".into()),
        },
        "importance": ?cx.str("importance").map(|i| i.to_lowercase()),
        "dueDateTime": ?dttz(cx.s("dueDateTime")),
        "reminderDateTime": ?dttz(cx.s("reminderDateTime")),
        "categories": ?cx.str("categories").map(|c| c.split(',').map(String::from).collect::<Vec<_>>()),
        "completedDateTime": ?dttz(cx.s("completedDateTime")),
        "startDateTime": ?dttz(cx.s("startDateTime")),
        "status": ?status,
    };
    let r = Request::post(format!("{LISTS}/{lid}/tasks"))
        .graph_json()
        .header("Content-Type", "application/json")
        .json(data)
        .data()
        .await?;
    cx.log(r)
}

fn trim_task(cx: &Cx, item: &Value) -> Value {
    if cx.str("output").as_deref() != Some("text") {
        return item.clone();
    }
    let g = |k: &str| item.get(k).cloned();
    obj! { "id": ?g("id"), "title": ?g("title"), "status": ?g("status"), "createdDateTime": ?g("createdDateTime"), "lastModifiedDateTime": ?g("lastModifiedDateTime") }
}

pub async fn task_get(cx: &mut Cx) -> Result<()> {
    let lid = todo_list_id(cx).await?;
    let item = Request::get(format!("{LISTS}/{lid}/tasks/{}", cx.u("id")))
        .graph_json()
        .data()
        .await?;
    cx.log(trim_task(cx, &item))
}

pub async fn task_list(cx: &mut Cx) -> Result<()> {
    let lid = todo_list_id(cx).await?;
    let items = odata::get_all_items(&format!("{LISTS}/{lid}/tasks")).await?;
    cx.log(Value::Array(
        items.iter().map(|i| trim_task(cx, i)).collect(),
    ))
}

pub async fn task_remove(cx: &mut Cx) -> Result<()> {
    let list = cx
        .str("listId")
        .or_else(|| cx.str("listName"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove the task {} from list {list}?",
            cx.u("id")
        ),
    )? {
        return Ok(());
    }
    let lid = match cx.s("listName") {
        Some(n) => single_list_id(&n).await?,
        None => cx.str("listId"),
    };
    let Some(lid) = lid.filter(|l| !l.is_empty()) else {
        return Err(Error::msg(format!(
            "The list {} cannot be found",
            cx.u("listName")
        )));
    };
    Request::delete(format!("{LISTS}/{lid}/tasks/{}", cx.u("id")))
        .graph_json()
        .send()
        .await?;
    Ok(())
}

pub async fn task_set(cx: &mut Cx) -> Result<()> {
    let mut b = Obj::new();
    let mut put = |k: &str, v: Option<Value>| {
        if let Some(v) = v {
            b.insert(k.into(), v);
        }
    };
    put("status", cx.s("status").map(Value::from));
    put("title", cx.s("title").map(Value::from));
    put(
        "importance",
        cx.s("importance").map(|i| Value::from(i.to_lowercase())),
    );
    if cx.s("bodyContentType").is_some() || cx.s("bodyContent").is_some() {
        put(
            "body",
            Some(obj! {
                "content": ?cx.val("bodyContent").cloned(),
                "contentType": cx.s("bodyContentType").map(|b| b.to_lowercase()).unwrap_or_else(|| "text".into()),
            }),
        );
    }
    put("dueDateTime", dttz(cx.s("dueDateTime")));
    put("reminderDateTime", dttz(cx.s("reminderDateTime")));
    put(
        "categories",
        cx.s("categories")
            .map(|c| Value::from(c.split(',').map(String::from).collect::<Vec<_>>())),
    );
    put("completedDateTime", dttz(cx.s("completedDateTime")));
    put("startDateTime", dttz(cx.s("startDateTime")));
    let lid = todo_list_id(cx).await?;
    let r = Request::patch(format!("{LISTS}/{lid}/tasks/{}", eqp(&cx.u("id"))))
        .graph_json()
        .header("Content-Type", "application/json")
        .json(Value::Object(b))
        .data()
        .await?;
    cx.log(r)
}
