//! Port of `src/m365/spo/commands/page/clientsidepages.ts`: the modern page
//! canvas model (parse `CanvasContent1`, edit sections and controls, and
//! write it back), matching upstream's markup and JSON byte for byte.

use m365_core::prelude::*;
use regex::Regex;
use std::sync::LazyLock;

/// `ClientSidePage.jsonToEscapedString`.
pub fn json_to_escaped(v: &Value) -> String {
    json::stringify(v)
        .replace('"', "&quot;")
        .replace(':', "&#58;")
        .replace('{', "&#123;")
        .replace('}', "&#125;")
}

/// `ClientSidePage.escapedStringToJson`.
pub fn escaped_to_json(s: Option<&str>) -> Result<Value> {
    let Some(s) = s.filter(|s| !s.is_empty()) else {
        return Ok(Value::Object(Obj::new()));
    };
    let mut r = s
        .replace('\\', "\\\\")
        .replace("&quot;", "\"")
        .replace("&#58;", ":")
        .replace("&#123;", "{")
        .replace("&#125;", "}")
        .replace("\\\\", "\\");
    for c in ['?', '.', '[', ']', '(', ')', '|', '+', '*', '$'] {
        r = r.replace(&format!("\\{c}"), &c.to_string());
    }
    json::jparse(&r)
}

/// `regexIndexOf.call(s, re, start)`.
fn regex_index_of(s: &str, re: &Regex, start: usize) -> Option<usize> {
    if start > s.len() {
        return None;
    }
    re.find(&s[start..]).map(|m| m.start() + start)
}

/// `getAttrValueFromString`: `null` when the attribute is missing.
fn attr(html: &str, re: &Regex) -> Option<String> {
    re.captures(html).map(|c| c[1].to_string())
}

/// The attributes `attr` reads, as `(?i)name="([^"]*?)"`.
fn attr_re(name: &str) -> Regex {
    Regex::new(&format!(r#"(?i){name}="([^"]*?)""#)).unwrap()
}
static WEBPARTDATA: LazyLock<Regex> = LazyLock::new(|| attr_re("data-sp-webpartdata"));
static CANVASDATAVERSION: LazyLock<Regex> = LazyLock::new(|| attr_re("data-sp-canvasdataversion"));
static WEBPARTDATAVERSION: LazyLock<Regex> =
    LazyLock::new(|| attr_re("data-sp-webpartdataversion"));
static CONTROLDATA: LazyLock<Regex> = LazyLock::new(|| attr_re("data-sp-controldata"));

/// `getBoundedDivMarkup`.
fn bounded_divs(html: &str, start_re: &Regex) -> Result<Vec<String>> {
    let clean: String = html
        .chars()
        .filter(|c| !matches!(c, '\t' | '\r' | '\n'))
        .collect();
    static OPEN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)<div[^>]*>").unwrap());
    static CLOSE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)</div>").unwrap());
    let mut out = Vec::new();
    let mut start = regex_index_of(&clean, start_re, 0);
    while let Some(si) = start {
        let mut counter = 1i32;
        let mut search = si + 1;
        let next_close;
        loop {
            let no = regex_index_of(&clean, &OPEN, search).unwrap_or(clean.len() + 1);
            let nc = regex_index_of(&clean, &CLOSE, search);
            let ncv = nc.map(|x| x as i64).unwrap_or(-1);
            if (no as i64) < ncv {
                counter += 1;
                search = no + 1;
            } else if ncv < no as i64 {
                counter -= 1;
                search = (ncv + 1) as usize;
            }
            if counter == 0 {
                let end = ((ncv + 6) as usize).min(clean.len());
                out.push(clean[si..end].trim().to_string());
                next_close = ncv;
                break;
            }
            if !(0..=1000).contains(&counter) {
                return Err(Error::js("getBoundedDivMarkup exceeded depth parameters."));
            }
        }
        start = if next_close < 0 {
            None
        } else {
            regex_index_of(&clean, start_re, next_close as usize)
        };
    }
    Ok(out)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Text,
    WebPart,
}

/// A `ClientSidePart` (text or web part) with its JS fields.
#[derive(Clone, Debug)]
pub struct Part {
    pub kind: Kind,
    pub control_type: Option<Value>,
    pub data_version: Option<Value>,
    pub order: Value,
    pub id: Option<Value>,
    pub control_data: Option<Value>,
    pub dynamic_data_paths: Value,
    pub dynamic_data_values: Value,
    pub text: String,
    pub title: Option<Value>,
    pub description: Option<Value>,
    pub propertie_json: Value,
    pub web_part_id: Option<Value>,
    pub html_properties: String,
    pub server_processed_content: Value,
    pub canvas_data_version: Option<Value>,
}

impl Part {
    pub fn text(text: &str) -> Part {
        let mut p = Part::base(Kind::Text, 4);
        p.set_text(text);
        p
    }
    pub fn webpart(title: &str) -> Part {
        let mut p = Part::base(Kind::WebPart, 3);
        p.title = Some(Value::from(title));
        p.description = Some(Value::from(""));
        p.web_part_id = Some(Value::from(""));
        p.dynamic_data_paths = Value::from("");
        p.dynamic_data_values = Value::from("");
        p.canvas_data_version = Some(Value::from("1.0"));
        p
    }
    fn base(kind: Kind, ty: i64) -> Part {
        Part {
            kind,
            control_type: Some(Value::from(ty)),
            data_version: Some(Value::from("1.0")),
            order: Value::from(1),
            id: Some(Value::from(util::new_guid())),
            control_data: None,
            dynamic_data_paths: Value::Null,
            dynamic_data_values: Value::Null,
            text: String::new(),
            title: None,
            description: None,
            propertie_json: Value::Object(Obj::new()),
            web_part_id: None,
            html_properties: String::new(),
            server_processed_content: Value::Null,
            canvas_data_version: None,
        }
    }
    /// `set text(t)`: wraps in `<p>` unless it already starts with one.
    pub fn set_text(&mut self, t: &str) {
        self.text = if t.starts_with("<p>") {
            t.to_string()
        } else {
            format!("<p>{t}</p>")
        };
    }
    fn read_html(&mut self, html: &str) -> Result<()> {
        base_from_html(
            html,
            &mut self.control_data,
            &mut self.data_version,
            &mut self.control_type,
            &mut self.id,
        )?;
        match self.kind {
            Kind::Text => {
                self.set_text("");
                static RTE: LazyLock<Regex> =
                    LazyLock::new(|| Regex::new(r"(?i)<div[^>]*data-sp-rte[^>]*>").unwrap());
                static INNER: LazyLock<Regex> = LazyLock::new(|| {
                    Regex::new(r"(?i)^<div[^>]*data-sp-rte[^>]*>(.*?)</div>$").unwrap()
                });
                for s in bounded_divs(html, &RTE)? {
                    let t = INNER
                        .captures(&s)
                        .map(|c| c[1].to_string())
                        .unwrap_or_default();
                    self.set_text(&t);
                }
            }
            Kind::WebPart => {
                let data = escaped_to_json(attr(html, &WEBPARTDATA).as_deref())?;
                self.title = data.get("title").cloned();
                self.description = data.get("description").cloned();
                self.web_part_id = data.get("id").cloned();
                let fix =
                    |s: Option<String>| Value::from(s.unwrap_or_default().replacen("\\.", ".", 1));
                self.canvas_data_version = Some(fix(attr(html, &CANVASDATAVERSION)));
                self.data_version = Some(fix(attr(html, &WEBPARTDATAVERSION)));
                extend(&mut self.propertie_json, data.get("properties"));
                if let Some(v) = data.get("serverProcessedContent") {
                    self.server_processed_content = v.clone();
                }
                if let Some(v) = data.get("dynamicDataPaths") {
                    self.dynamic_data_paths = v.clone();
                }
                if let Some(v) = data.get("dynamicDataValues") {
                    self.dynamic_data_values = v.clone();
                }
                static HP: LazyLock<Regex> = LazyLock::new(|| {
                    Regex::new(r"(?i)<div\b[^>]*data-sp-htmlproperties[^>]*?>").unwrap()
                });
                static STRIP_OPEN: LazyLock<Regex> = LazyLock::new(|| {
                    Regex::new(r"(?i)^<div\b[^>]*data-sp-htmlproperties[^>]*?>").unwrap()
                });
                static STRIP_CLOSE: LazyLock<Regex> =
                    LazyLock::new(|| Regex::new(r"(?i)</div>$").unwrap());
                let props = bounded_divs(html, &HP)?;
                self.html_properties = props
                    .first()
                    .map(|m| {
                        STRIP_CLOSE
                            .replace(&STRIP_OPEN.replace(m, ""), "")
                            .into_owned()
                    })
                    .unwrap_or_default();
            }
        }
        Ok(())
    }
    fn position(&self, col: Option<(&Column, &Section)>) -> Obj {
        let mut p = Obj::new();
        p.insert("controlIndex".into(), self.order.clone());
        p.insert(
            "sectionFactor".into(),
            col.map(|(c, _)| c.factor.clone()).unwrap_or(Value::from(0)),
        );
        p.insert(
            "sectionIndex".into(),
            col.map(|(c, _)| c.order.clone()).unwrap_or(Value::from(0)),
        );
        p.insert(
            "zoneIndex".into(),
            col.map(|(_, s)| s.order.clone()).unwrap_or(Value::from(0)),
        );
        if let Some((_, s)) = col {
            p.insert("zoneId".into(), s.zone_id.clone());
            p.insert("layoutIndex".into(), s.layout_index.clone());
        }
        p
    }
    /// `getControlData()` for the part in its column/section.
    pub fn control_data_for(&self, col: Option<(&Column, &Section)>) -> Value {
        let mut o = Obj::new();
        if let Some(t) = &self.control_type {
            o.insert("controlType".into(), t.clone());
        }
        if self.kind == Kind::Text {
            o.insert("editorType".into(), Value::from("CKEditor"));
        }
        if let Some(i) = &self.id {
            o.insert("id".into(), i.clone());
        }
        let mut pos = self.position(col);
        o.insert("position".into(), Value::Null);
        if self.kind == Kind::WebPart
            && let Some(w) = &self.web_part_id
        {
            o.insert("webPartId".into(), w.clone());
        }
        if let Some((_, s)) = col {
            if let Some(z) = &s.zone_group_metadata {
                o.insert("zoneGroupMetadata".into(), z.clone());
            }
            if let Some(e) = &s.emphasis {
                o.insert("emphasis".into(), e.clone());
            }
            if let Some(r) = &s.is_layout_reflow_on_top {
                pos.insert("isLayoutReflowOnTop".into(), r.clone());
            }
        }
        o.insert("position".into(), Value::Object(pos));
        Value::Object(o)
    }
    fn render(&mut self, index: usize, col: Option<(&Column, &Section)>) -> String {
        self.order = Value::from(index);
        let jd = json_to_escaped(&self.control_data_for(col));
        match self.kind {
            Kind::Text => format!(
                r#"<div data-sp-canvascontrol="" data-sp-canvasdataversion="{}" data-sp-controldata="{jd}"><div data-sp-rte="">{}</div></div>"#,
                js_str(&self.data_version),
                self.text
            ),
            Kind::WebPart => {
                let mut data = obj! {
                    "dataVersion": ?self.data_version.clone(), "description": ?self.description.clone(), "id": ?self.web_part_id.clone(),
                    "instanceId": ?self.id.clone(), "properties": self.propertie_json.clone(), "serverProcessedContent": self.server_processed_content.clone(),
                    "title": ?self.title.clone(),
                };
                if truthy(&self.dynamic_data_paths) {
                    data["dynamicDataPaths"] = self.dynamic_data_paths.clone();
                }
                if truthy(&self.dynamic_data_values) {
                    data["dynamicDataValues"] = self.dynamic_data_values.clone();
                }
                format!(
                    r#"<div data-sp-canvascontrol="" data-sp-canvasdataversion="{}" data-sp-controldata="{jd}"><div data-sp-webpart="" data-sp-webpartdataversion="{}" data-sp-webpartdata="{}"><div data-sp-componentid>{}</div><div data-sp-htmlproperties="">{}</div></div></div>"#,
                    js_str(&self.canvas_data_version),
                    js_str(&self.data_version),
                    json_to_escaped(&data),
                    js_str(&self.web_part_id),
                    self.render_html_properties()
                )
            }
        }
    }
    fn render_html_properties(&self) -> String {
        let spc = &self.server_processed_content;
        if spc.is_null() {
            return self.html_properties.clone();
        }
        let mut h = String::new();
        if let Some(Value::Object(o)) = spc.get("searchablePlainTexts") {
            for (k, v) in o {
                h.push_str(&format!(
                    r#"<div data-sp-prop-name="{k}" data-sp-searchableplaintext="true">{}</div>"#,
                    to_js_string(v)
                ));
            }
        }
        if let Some(Value::Object(o)) = spc.get("imageSources") {
            for (k, v) in o {
                h.push_str(&format!(
                    r#"<img data-sp-prop-name="{k}" src="{}" />"#,
                    to_js_string(v)
                ));
            }
        }
        if let Some(Value::Object(o)) = spc.get("links") {
            for (k, v) in o {
                h.push_str(&format!(
                    r#"<a data-sp-prop-name="{k}" href="{}"></a>"#,
                    to_js_string(v)
                ));
            }
        }
        h
    }
    /// `Page.getControlsInformation`: the instance as JSON without `column`.
    pub fn info(&self, json_output: bool) -> Value {
        let mut o = Obj::new();
        let ct = self.control_type.clone().map(|c| {
            if json_output {
                c
            } else {
                Value::from(control_type_name(&c))
            }
        });
        if let Some(c) = ct {
            o.insert("controlType".into(), c);
        }
        if let Some(d) = &self.data_version {
            o.insert("dataVersion".into(), d.clone());
        }
        o.insert("order".into(), self.order.clone());
        if let Some(i) = &self.id {
            o.insert("id".into(), i.clone());
        }
        if let Some(c) = &self.control_data {
            o.insert("controlData".into(), c.clone());
        }
        if truthy(&self.dynamic_data_paths) {
            o.insert("dynamicDataPaths".into(), self.dynamic_data_paths.clone());
        }
        if truthy(&self.dynamic_data_values) {
            o.insert("dynamicDataValues".into(), self.dynamic_data_values.clone());
        }
        match self.kind {
            Kind::Text => {
                o.insert("_text".into(), Value::from(self.text.as_str()));
            }
            Kind::WebPart => {
                for (k, v) in [("title", &self.title), ("description", &self.description)] {
                    if let Some(v) = v {
                        o.insert(k.into(), v.clone());
                    }
                }
                o.insert("propertieJson".into(), self.propertie_json.clone());
                if let Some(w) = &self.web_part_id {
                    o.insert("webPartId".into(), w.clone());
                }
                o.insert(
                    "htmlProperties".into(),
                    Value::from(self.html_properties.as_str()),
                );
                o.insert(
                    "serverProcessedContent".into(),
                    self.server_processed_content.clone(),
                );
                if let Some(c) = &self.canvas_data_version {
                    o.insert("canvasDataVersion".into(), c.clone());
                }
            }
        }
        Value::Object(o)
    }
}

/// `getControlTypeDisplayName`.
pub fn control_type_name(v: &Value) -> String {
    match v.as_f64() {
        Some(0.0) => "Empty column".into(),
        Some(3.0) => "Client-side web part".into(),
        Some(4.0) => "Client-side text".into(),
        _ => to_js_string(v),
    }
}

fn js_str(v: &Option<Value>) -> String {
    v.as_ref()
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into())
}

/// `extend(target, source)`.
fn extend(target: &mut Value, source: Option<&Value>) {
    if let (Some(Value::Object(src)), Value::Object(t)) = (source, target) {
        for (k, v) in src {
            t.insert(k.clone(), v.clone());
        }
    }
}

fn base_from_html(
    html: &str,
    cd: &mut Option<Value>,
    dv: &mut Option<Value>,
    ct: &mut Option<Value>,
    id: &mut Option<Value>,
) -> Result<()> {
    let data = escaped_to_json(attr(html, &CONTROLDATA).as_deref())?;
    *dv = Some(
        attr(html, &CANVASDATAVERSION)
            .map(Value::from)
            .unwrap_or(Value::Null),
    );
    *ct = data.get("controlType").cloned();
    *id = data.get("id").cloned();
    *cd = Some(data);
    Ok(())
}

#[derive(Clone, Debug)]
pub struct Column {
    pub order: Value,
    pub factor: Value,
    pub data_version: Option<Value>,
    pub id: Option<Value>,
    pub control_data: Option<Value>,
    pub controls: Vec<Part>,
}

impl Column {
    fn new(order: Value, factor: Value) -> Column {
        Column {
            order,
            factor,
            data_version: Some(Value::from("1.0")),
            id: Some(Value::from(util::new_guid())),
            control_data: None,
            controls: Vec::new(),
        }
    }
    /// `getControlData()` (`this.column` is always undefined on a column).
    pub fn control_data_for(&self, s: &Section) -> Value {
        let mut pos = Obj::new();
        pos.insert("sectionFactor".into(), self.factor.clone());
        pos.insert("sectionIndex".into(), self.order.clone());
        pos.insert(
            "zoneIndex".into(),
            if truthy(&s.order) {
                s.order.clone()
            } else {
                Value::from(0)
            },
        );
        pos.insert("zoneId".into(), s.zone_id.clone());
        pos.insert("layoutIndex".into(), s.layout_index.clone());
        let mut o = Obj::new();
        o.insert("position".into(), Value::Object(pos));
        if let Some(z) = &s.zone_group_metadata {
            o.insert("zoneGroupMetadata".into(), z.clone());
        }
        if let Some(e) = &s.emphasis {
            o.insert("emphasis".into(), e.clone());
        }
        if self.controls.is_empty() {
            o.insert("id".into(), Value::from("emptySection"));
            o.insert("controlType".into(), Value::from(1));
        }
        Value::Object(o)
    }
    fn render(&mut self, s: &Section) -> String {
        if self.controls.is_empty() {
            return format!(
                r#"<div data-sp-canvascontrol="" data-sp-canvasdataversion="{}" data-sp-controldata="{}"></div>"#,
                js_str(&self.data_version),
                json_to_escaped(&self.control_data_for(s))
            );
        }
        let me = self.clone();
        let mut out = String::new();
        for (i, c) in self.controls.iter_mut().enumerate() {
            out.push_str(&c.render(i + 1, Some((&me, s))));
        }
        out
    }
    /// `Page.getColumnsInformation`.
    pub fn info(&self, s: &Section, json_output: bool) -> Value {
        let mut o = obj! { "factor": self.factor.clone(), "order": self.order.clone() };
        if json_output {
            if let Some(d) = &self.data_version {
                o["dataVersion"] = d.clone();
            }
            o["jsonData"] = Value::from(json_to_escaped(&self.control_data_for(s)));
        }
        o
    }
}

#[derive(Clone, Debug)]
pub struct Section {
    pub order: Value,
    pub columns: Vec<Column>,
    pub control_data: Option<Value>,
    pub zone_id: Value,
    pub zone_group_metadata: Option<Value>,
    pub emphasis: Option<Value>,
    pub layout_index: Value,
    pub is_layout_reflow_on_top: Option<Value>,
}

impl Section {
    pub fn new(order: Value, control_data: Option<Value>) -> Section {
        let pos = control_data.as_ref().and_then(|c| c.get("position"));
        Section {
            order,
            columns: Vec::new(),
            zone_id: pos
                .and_then(|p| p.get("zoneId"))
                .filter(|z| truthy(z))
                .cloned()
                .unwrap_or_else(|| Value::from(util::new_guid())),
            zone_group_metadata: control_data
                .as_ref()
                .and_then(|c| c.get("zoneGroupMetadata"))
                .cloned(),
            emphasis: control_data
                .as_ref()
                .and_then(|c| c.get("emphasis"))
                .cloned(),
            layout_index: pos
                .and_then(|p| p.get("layoutIndex"))
                .filter(|v| !v.is_null())
                .cloned()
                .unwrap_or(Value::from(1)),
            is_layout_reflow_on_top: pos.and_then(|p| p.get("isLayoutReflowOnTop")).cloned(),
            control_data,
        }
    }
    /// `Page.getSectionInformation`.
    pub fn info(&self, json_output: bool) -> Value {
        let mut o = obj! { "order": self.order.clone() };
        let factor = self
            .control_data
            .as_ref()
            .and_then(|c| c.get("position"))
            .and_then(|p| p.get("sectionFactor"))
            .and_then(Value::as_f64);
        if self.layout_index.as_f64() == Some(2.0) && factor == Some(12.0) {
            o["isVertical"] = Value::Bool(true);
        }
        o["columns"] = Value::Array(
            self.columns
                .iter()
                .map(|c| c.info(self, json_output))
                .collect(),
        );
        o
    }
    pub fn add_column(&mut self, factor: Value) {
        let next = self
            .columns
            .iter()
            .filter_map(|c| c.order.as_f64())
            .fold(f64::NAN, f64::max);
        let order = if self.columns.is_empty() {
            1.0
        } else {
            next + 1.0
        };
        self.columns
            .push(Column::new(json::num_value(order), factor));
    }
}

/// Page settings and background controls (kept verbatim).
#[derive(Clone, Debug)]
pub struct Other {
    pub control_type: Option<Value>,
    pub data_version: Option<Value>,
    pub id: Option<Value>,
    pub control_data: Option<Value>,
    pub propertie_json: Value,
    pub server_processed_content: Value,
}

#[derive(Clone, Debug, Default)]
pub struct ClientSidePage {
    pub sections: Vec<Section>,
    pub page_settings: Option<Other>,
    pub background: Option<Other>,
}

fn position_of(cd: &Option<Value>, key: &str) -> Result<Option<Value>> {
    match cd {
        None => Ok(None),
        Some(c) => match c.get("position") {
            Some(p) => Ok(p.get(key).cloned()),
            None => Err(Error::js(format!(
                "Cannot read properties of undefined (reading '{key}')"
            ))),
        },
    }
}

impl ClientSidePage {
    /// `ClientSidePage.fromHtml`.
    pub fn from_html(html: &str) -> Result<ClientSidePage> {
        let mut page = ClientSidePage::default();
        static START: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)<div\b[^>]*data-sp-canvascontrol[^>]*?>").unwrap());
        static CONTROL_TYPE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)controlType&quot;&#58;(\d*?)(,|&)").unwrap());
        for markup in bounded_divs(html, &START)? {
            let ct = match CONTROL_TYPE.captures(&markup) {
                None => -1,
                Some(c) => c[1].parse::<i64>().unwrap_or(i64::MIN),
            };
            match ct {
                -1 | 1 => {
                    let mut col = Column::new(Value::from(0), Value::from(12));
                    let (mut cd, mut dv, mut cty, mut id) = (None, None, None, None);
                    base_from_html(&markup, &mut cd, &mut dv, &mut cty, &mut id)?;
                    col.control_data = cd;
                    col.data_version = dv;
                    col.id = id;
                    if let Some(p) = col.control_data.as_ref().and_then(|c| c.get("position")) {
                        if let Some(f) = p.get("sectionFactor") {
                            col.factor = f.clone();
                        }
                        if let Some(o) = p.get("sectionIndex") {
                            col.order = o.clone();
                        }
                    }
                    page.merge_column(col)?;
                }
                0 | 14 => {
                    let (mut cd, mut dv, mut cty, mut id) = (None, None, None, None);
                    base_from_html(&markup, &mut cd, &mut dv, &mut cty, &mut id)?;
                    let mut o = Other {
                        control_type: cty,
                        data_version: dv,
                        id,
                        control_data: cd,
                        propertie_json: Value::Object(Obj::new()),
                        server_processed_content: Value::Null,
                    };
                    if ct == 0 {
                        page.page_settings = Some(o);
                    } else {
                        let data = escaped_to_json(attr(&markup, &WEBPARTDATA).as_deref())?;
                        extend(&mut o.propertie_json, data.get("properties"));
                        if let Some(v) = data.get("serverProcessedContent") {
                            o.server_processed_content = v.clone();
                        }
                        page.background = Some(o);
                    }
                }
                3 | 4 => {
                    let mut p = if ct == 3 {
                        Part::webpart("")
                    } else {
                        Part::text("")
                    };
                    p.read_html(&markup)?;
                    page.merge_part(p)?;
                }
                _ => {}
            }
        }
        page.reindex();
        Ok(page)
    }

    fn find_section(&self, order: &Value, layout: &Value) -> Option<usize> {
        self.sections
            .iter()
            .position(|s| &s.order == order && &s.layout_index == layout)
    }

    fn merge_part(&mut self, part: Part) -> Result<()> {
        let pos = part.control_data.as_ref().and_then(|c| c.get("position"));
        let zone = pos
            .and_then(|p| p.get("zoneIndex"))
            .cloned()
            .unwrap_or(Value::from(0));
        let sidx = pos
            .and_then(|p| p.get("sectionIndex"))
            .cloned()
            .unwrap_or(Value::from(0));
        let factor = pos
            .and_then(|p| p.get("sectionFactor"))
            .cloned()
            .unwrap_or(Value::from(12));
        let layout = position_of(&part.control_data, "layoutIndex")?
            .filter(|v| !v.is_null())
            .unwrap_or(Value::from(1));
        let si = match self.find_section(&zone, &layout) {
            Some(i) => i,
            None => {
                self.sections
                    .push(Section::new(zone, part.control_data.clone()));
                self.sections.len() - 1
            }
        };
        let s = &mut self.sections[si];
        let ci = match s.columns.iter().position(|c| c.order == sidx) {
            Some(i) => i,
            None => {
                s.columns.push(Column::new(sidx, factor));
                s.columns.len() - 1
            }
        };
        s.columns[ci].controls.push(part);
        Ok(())
    }

    fn merge_column(&mut self, col: Column) -> Result<()> {
        let zone = position_of(&col.control_data, "zoneIndex")?
            .filter(truthy)
            .unwrap_or(Value::from(0));
        let layout = position_of(&col.control_data, "layoutIndex")?
            .filter(|v| !v.is_null())
            .unwrap_or(Value::from(1));
        let si = match self.find_section(&zone, &layout) {
            Some(i) => i,
            None => {
                self.sections
                    .push(Section::new(zone, col.control_data.clone()));
                self.sections.len() - 1
            }
        };
        self.sections[si].columns.push(col);
        Ok(())
    }

    /// `reindex(this.sections)`.
    pub fn reindex(&mut self) {
        for (i, s) in self.sections.iter_mut().enumerate() {
            s.order = Value::from(i + 1);
            for (j, c) in s.columns.iter_mut().enumerate() {
                c.order = Value::from(j + 1);
                for (k, p) in c.controls.iter_mut().enumerate() {
                    p.order = Value::from(k + 1);
                }
            }
        }
    }

    /// `ClientSidePage.toHtml`.
    pub fn to_html(&mut self) -> String {
        self.reindex();
        let mut h = String::from("<div>");
        for s in self.sections.iter_mut() {
            let snapshot = s.clone();
            for c in s.columns.iter_mut() {
                h.push_str(&c.render(&snapshot));
            }
        }
        if let Some(p) = &self.page_settings {
            h.push_str(&format!(r#"<div data-sp-canvascontrol="" data-sp-canvasdataversion="{}" data-sp-controldata="{}"></div>"#, js_str(&p.data_version), json_to_escaped(p.control_data.as_ref().unwrap_or(&Value::Null))));
        }
        if let Some(b) = &self.background {
            let data = obj! { "dataVersion": ?b.data_version.clone(), "instanceId": ?b.id.clone(), "properties": b.propertie_json.clone(), "serverProcessedContent": b.server_processed_content.clone() };
            let cd = obj! { "controlType": ?b.control_type.clone() };
            h.push_str(&format!(
                r#"<div data-sp-canvascontrol="" data-sp-canvasdataversion="{}" data-sp-controldata="{}"><div data-sp-webpart="" data-sp-webpartdataversion="{}" data-sp-webpartdata="{}"><div data-sp-componentid=""></div><div data-sp-htmlproperties="">"#,
                js_str(&b.data_version),
                json_to_escaped(&cd),
                js_str(&b.data_version),
                json_to_escaped(&data)
            ));
            if let Some(Value::Object(o)) = b.server_processed_content.get("imageSources") {
                for (k, v) in o {
                    h.push_str(&format!(
                        r#"<img data-sp-prop-name="{k}" src="{}" />"#,
                        to_js_string(v)
                    ));
                }
            }
            h.push_str("</div></div></div>");
        }
        h.push_str("</div>");
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Upstream's `ClientSidePage.fromHtml(canvas).toHtml()` for the parity
    /// fixture (page settings, two-column text section, emphasised
    /// collapsible web part section, vertical text section, background).
    #[test]
    fn round_trip_matches_upstream() {
        let canvas = include_str!("../tests/page-canvas.html").trim_end();
        let expected = include_str!("../tests/page-canvas.expected.html").trim_end();
        let mut p = ClientSidePage::from_html(canvas).unwrap();
        assert_eq!(p.to_html(), expected);
    }
}
