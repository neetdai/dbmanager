
use serde_json::Value;
use tauri::{
    menu::{CheckMenuItem, IsMenuItem, Menu, MenuBuilder, MenuId, MenuItem, SubmenuBuilder}, App, AppHandle, Event, EventLoopMessage, Listener, Manager, Result, Runtime, WebviewWindowBuilder, Window, Wry
};

use std::{collections::hash_map::Values, sync::{Arc, OnceLock}};
use std::collections::HashMap;

#[derive(Clone)]
struct Languages {
    inner: Arc<OnceLock<HashMap<&'static str, CheckMenuItem<Wry>>>>,
}

impl Languages {
    fn new(handle: &AppHandle) -> Result<Self> {
        let mut inner = HashMap::new();
        inner.insert("language_english", CheckMenuItem::with_id(handle, "language_english", "English", true, false,None::<&str>)?);
        inner.insert("language_chinese", CheckMenuItem::with_id(handle, "language_chinese",  "简体中文", true, true, None::<&str>)?);
        
        let lock = OnceLock::new();
        lock.set(inner);

        Ok(Self {
            inner: Arc::new(lock),
        })
    }

    fn checked(&self, id: &str) {
        let map = self.inner.get().unwrap();

        if map.contains_key(id) {
            for (_, item) in map.iter() {
                item.set_checked(false);
            }

            map.get(id).unwrap().set_checked(true);
        }
    }

    fn list(&self) -> Values<'_, &'static str,  CheckMenuItem<Wry>> {
        self.inner.get().unwrap().values()
    }

    fn contain(&self, id: &str) -> bool {
        self.inner.get().unwrap().contains_key(id)
    }
}

pub(crate) fn build_menu<'a>(app: &'a mut App) -> Result<()> {
    let handle = app.handle();

    let new_connection_menu = MenuItem::with_id(handle,"new_connection", "新建连接", true, None::<&str>)?;
    let close_connection_menu = MenuItem::with_id(handle,"close_connection", "关闭连接", true, None::<&str>)?;
    let languages = Languages::new(handle)?;

    let connection_menu = SubmenuBuilder::new(handle, "连接")
        .items(&[
            &new_connection_menu,
            &close_connection_menu,
        ])
        .build()?;

    let change_language = SubmenuBuilder::new(handle, "语言").build()?;

    for item in languages.list() {
        change_language.append(item)?;        
    }

    let setting_menu = SubmenuBuilder::new(handle, "设置").items(&[]).build()?;

    setting_menu.append_items(&[
        &change_language,
    ])?;

    let menu = MenuBuilder::new(handle).build()?;

    menu.append_items(&[&connection_menu, &setting_menu])?;

    handle.on_menu_event(move |handle, event| {
        let id = event.id();
        if id == "new_connection" {
            build_new_connection_form(handle).unwrap();
        } else if id == "close_connection" {
            
        } else if languages.contain(id.0.as_str()) {
            languages.checked(id.0.as_str());
        }
    });

    app.set_menu(menu)?;

    Ok(())
}

pub(crate) fn build_new_connection_form(handle: &AppHandle) -> Result<()> {
    let window = WebviewWindowBuilder::new(handle, "new_connection", tauri::WebviewUrl::App("/new_connection".into())).build()?;

    window.hide_menu()?;

    Ok(())
}