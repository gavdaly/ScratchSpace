use resvg::Tree as RTree;
use std::sync::Arc;
use tiny_skia::Pixmap;
use usvg::{ImageHrefResolver, ImageKind, Options, Tree, TreeParsing};

pub fn genreate(template: &str, width: u32, height: u32) -> Vec<u8> {
    let template = std::fs::read_to_string(template).unwrap();

    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(&template)
        .unwrap();

    let globals = liquid::object!({
        "text": "test"
    });

    let svg = template.render(&globals).unwrap();

    let mut image = Pixmap::new(width, height).unwrap();

    let options = Options {
        image_href_resolver: ImageHrefResolver {
            resolve_string: Box::new(move |path: &str, _| {
                let response = reqwest::blocking::get(path).ok()?;
                let content_type = response
                    .headers()
                    .get("content-type")
                    .and_then(|hv| hv.to_str().ok())?
                    .to_owned();
                let image_buffer = response.bytes().ok()?.into_iter().collect::<Vec<u8>>();
                match content_type.as_str() {
                    "image/png" => Some(ImageKind::PNG(Arc::new(image_buffer))),
                    _ => None,
                }
            }),
            ..Default::default()
        },
        ..Default::default()
    };

    let tree: Tree = TreeParsing::from_str(&svg, &options).unwrap();

    RTree::from_usvg(&tree).render(tiny_skia::Transform::default(), &mut image.as_mut());

    let encoded = webp::Encoder::new(
        image.as_ref().data(),
        webp::PixelLayout::Rgba,
        width,
        height,
    )
    .encode_lossless();
    encoded.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        genreate("Template.svg", 1200, 630);
    }
}
