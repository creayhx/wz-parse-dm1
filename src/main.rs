use wz_reader::{property::get_image, util::walk_node, WzNode, WzNodeArc};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The img file version is CMS 085
    let map_img_path = "./100000000.img";

    let node: WzNodeArc = WzNode::from_img_file(map_img_path, None, None)?.into();

    walk_node(&node, true, &|node| {
        let node_read = node.read().unwrap();

        if node_read.name.contains("canvas") {
            let img = get_image(node); // -> Err(UnsupportedHeader(21576))
            println!("result: {:?}", img);
        }
    });

    Ok(())
}
