use std::error::Error;
use std::result::Result as StdResult;
use visdom::Vis;
type Result = StdResult<(), Box<dyn Error>>;

#[test]
fn test_dom_remove_child() -> Result {
	const HTML: &str = r#"<div><p></p><ul></ul><ol></ol></div>"#;
	let root = Vis::load(HTML)?;
	let div = root.children("div");
	let p = div.children("p");
	let ul = div.children("ul");
	let ol = div.children("ol");
	// remove before
	assert_eq!(0, p.get(0).unwrap().index());
	assert_eq!(1, ul.get(0).unwrap().index());
	assert_eq!(2, ol.get(0).unwrap().index());
	p.remove();
	assert_eq!(0, ul.get(0).unwrap().index());
	assert_eq!(1, ol.get(0).unwrap().index());
	// remove center
	let root = Vis::load(HTML)?;
	let div = root.children("div");
	let p = div.children("p");
	let ul = div.children("ul");
	let ol = div.children("ol");
	ul.remove();
	assert_eq!(0, p.get(0).unwrap().index());
	assert_eq!(1, ol.get(0).unwrap().index());
	// remove after
	let root = Vis::load(HTML)?;
	let div = root.children("div");
	let p = div.children("p");
	let ul = div.children("ul");
	let ol = div.children("ol");
	ol.remove();
	assert_eq!(0, p.get(0).unwrap().index());
	assert_eq!(1, ul.get(0).unwrap().index());
	Ok(())
}

#[test]
fn test_dom_append_child() -> Result {
	const HTML: &str = r#"<div class="parent"><div class="first-child"></div></div>"#;
	let root = Vis::load(HTML)?;
	let mut parent = root.children(".parent");
	let first_child = parent.children(".first-child");
	let mut new_childs =
		Vis::load(r#"<div class="second-child"></div><div class="third-child"></div>"#)?;
	assert_eq!(0, first_child.get(0).unwrap().index());
	parent.append(&mut new_childs);
	assert_eq!(0, first_child.get(0).unwrap().index());
	let all_childs = parent.children("");
	let last_child = all_childs.eq(all_childs.length() - 1);
	assert_eq!(2, last_child.get(0).unwrap().index());
	Ok(())
}

#[test]
fn test_dom_prepend_child() -> Result {
	const HTML: &str = r#"<div class="parent"><div class="third-child"></div></div>"#;
	let root = Vis::load(HTML)?;
	let mut parent = root.children(".parent");
	let last_child = parent.children(".third-child");
	let mut new_childs =
		Vis::load(r#"<div class="first-child"></div><div class="second-child"></div>"#)?;
	assert_eq!(0, last_child.get(0).unwrap().index());
	parent.prepend(&mut new_childs);
	assert_eq!(2, last_child.get(0).unwrap().index());
	let all_childs = parent.children("");
	let first_child = all_childs.eq(0);
	assert_eq!(0, first_child.get(0).unwrap().index());
	Ok(())
}

#[test]
fn test_dom_insert_before() -> Result {
	const HTML: &str = r#"<div class="parent"><div class="third-child"></div></div>"#;
	let root = Vis::load(HTML)?;
	let parent = root.children(".parent");
	let mut third_child = parent.children(".third-child");
	let inserted = Vis::load(r#"<div class="first-child"></div><div class="second-child"></div>"#)?;
	let inserted_childs = inserted.children("");
	assert_eq!(0, third_child.get(0).unwrap().index());
	// append second child
	let mut second_child = inserted_childs.filter(".second-child");
	second_child.insert_before(&mut third_child);
	assert_eq!(1, third_child.get(0).unwrap().index());
	assert_eq!(0, second_child.get(0).unwrap().index());
	assert_eq!(1, inserted.children("").length());
	// append first_child
	let mut first_child = inserted_childs.filter(".first-child");
	first_child.insert_before(&mut second_child);
	assert_eq!(2, third_child.get(0).unwrap().index());
	assert_eq!(1, second_child.get(0).unwrap().index());
	assert_eq!(0, first_child.get(0).unwrap().index());
	assert_eq!(0, inserted.children("").length());
	Ok(())
}