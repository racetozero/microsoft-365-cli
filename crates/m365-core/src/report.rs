//! `PeriodBasedReport` and `DateAndPeriodBasedReport`: the Graph usage
//! report commands, one implementation for all of them.

use crate::prelude::*;

/// `executeReport`.
pub async fn execute(cx: &Cx, endpoint: &str) -> Result<()> {
    let res = Request::get(endpoint).graph_json().data().await?;
    let text = match &res {
        Value::String(s) => s.clone(),
        // `removeEmptyLines(input)` calls `input.split`
        _ => return Err(Error::js("input.split is not a function")),
    };
    let clean: Vec<&str> = text.split('\n').filter(|r| !r.is_empty()).collect();
    let clean = clean.join("\n");
    if cx.str("output").is_some_and(|o| o.to_lowercase() == "json") {
        let rows: Vec<&str> = clean.split('\n').collect();
        let headers: Vec<&str> = rows[0].split(',').collect();
        let mut out = Vec::new();
        for row in &rows[1..] {
            let mut o = Obj::new();
            for (j, d) in row.split(',').enumerate() {
                let key = headers.get(j).map(|h| h.trim()).unwrap_or("undefined");
                o.insert(key.to_string(), Value::from(d.trim()));
            }
            out.push(Value::Object(o));
        }
        cx.log(Value::Array(out))
    } else {
        cx.log(Value::from(clean))
    }
}

pub async fn run(cx: &mut Cx) -> Result<()> {
    let ep = cx.meta.usage_endpoint.unwrap_or("");
    let endpoint = if cx.meta.is(crate::manifest::base::DATE_PERIOD_REPORT) {
        let p = match cx.s("period") {
            Some(p) => format!("{ep}(period='{}')", eqp(&p)),
            None => format!("{ep}(date={})", eqp(&cx.u("date"))),
        };
        format!("{GRAPH}/v1.0/reports/{p}")
    } else {
        format!(
            "{GRAPH}/v1.0/reports/{ep}(period='{}')",
            eqp(&cx.u("period"))
        )
    };
    execute(cx, &endpoint).await
}

/// Specs for every report command that has no own implementation.
pub fn specs() -> Vec<crate::cx::Spec> {
    crate::manifest::COMMANDS
        .iter()
        .filter(|c| c.usage_endpoint.is_some() && c.is(crate::manifest::base::PERIOD_REPORT))
        .map(|c| {
            if c.is(crate::manifest::base::DATE_PERIOD_REPORT) {
                crate::spec!(c.name, run, refine = not_both)
            } else {
                crate::spec!(c.name, run)
            }
        })
        .collect()
}

/// `!(opts.period !== undefined && opts.date !== undefined)`.
fn not_both(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    Ok(!(o.contains_key("period") && o.contains_key("date")))
}
