# temporary

```
use std::path::Path;

assert_eq!("rs", Path::new("foo.rs").extension().unwrap());
assert_eq!("gz", Path::new("foo.tar.gz").extension().unwrap());
```

```
        path.as_ref()
            .extension()
            .and_then(|e| e.to_str())
            .ok_or(AssetServerError::MissingAssetLoader(None))
            .and_then(|extension| self.get_asset_loader(extension))
```