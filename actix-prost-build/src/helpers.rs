use syn::{GenericArgument, Path, PathArguments, PathSegment};

pub fn extract_type_from_option(ty: &syn::Type) -> Option<&syn::Type> {
    fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
        let idents_of_path = path.segments.iter().fold(String::new(), |mut acc, v| {
            acc.push_str(&v.ident.to_string());
            acc.push('|');
            acc
        });
        vec!["Option|", "std|option|Option|", "core|option|Option|"]
            .into_iter()
            .find(|s| &idents_of_path == s)
            .and_then(|_| path.segments.last())
    }

    extract_type_path(ty)
        .and_then(|path| extract_option_segment(path))
        .and_then(|path_seg| {
            let type_params = &path_seg.arguments;
            // It should have only on angle-bracketed param ("<String>"):
            match *type_params {
                PathArguments::AngleBracketed(ref params) => params.args.first(),
                _ => None,
            }
        })
        .and_then(|generic_arg| match *generic_arg {
            GenericArgument::Type(ref ty) => Some(ty),
            _ => None,
        })
}

pub fn try_construct_external_type_path(path: &Path) -> Option<Path> {
    let idents_of_path = path.segments.iter().fold(String::new(), |mut acc, v| {
        acc.push_str(&v.ident.to_string());
        acc.push('|');
        acc
    });

    // ignore conventional external imported types
    let ignore_segments = ["super|google|protobuf|"];
    if ignore_segments.iter().any(|s| idents_of_path.contains(s)) {
        return None;
    }

    if !idents_of_path.starts_with("super|") {
        return None;
    }

    let mut new_path = path.clone();

    let ty = new_path.segments.last_mut()?;
    ty.ident = quote::format_ident!("{}Internal", ty.ident);

    Some(new_path)
}

fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
    match *ty {
        syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
        _ => None,
    }
}
