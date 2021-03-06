# projects


## windows:

### windows dialog


Select a directory:

```
//extern crate wfd;

use wfd::{DialogParams, FOS_PICKFOLDERS};

fn main() {
    let params = DialogParams {
        options: FOS_PICKFOLDERS,
        title: "Select a directory",
        ..Default::default()
    };

    let result = wfd::open_dialog(params);
    println!("Selected Folder: {}", result.unwrap().selected_file_path.to_str().unwrap());
}

```

```
use wfd::{DialogParams, FOS_ALLOWMULTISELECT, DialogError};

fn main() {
    let params = DialogParams {
        file_types: vec![("DLL Files", "*.dll"), ("Executable Files", "*.exe;*.com;*.scr")],
        default_extension: "dll",
        default_folder: r"C:\Windows\System32",
        file_name: "win32k.sys",
        file_name_label: "Select some files!",
        file_type_index: 1,
        ok_button_label: "Custom OK",
        options: FOS_ALLOWMULTISELECT,
        title: "Test open file dialog",
        .. Default::default()
    };

    match wfd::open_dialog(params) {
        Ok(r) => {
            for file in r.selected_file_paths {
                println!("{}", file.to_str().unwrap());
            }
        }
        Err(e) => match e {
            DialogError::UserCancelled => {
                println!("User cancelled dialog");
            }
            DialogError::HResultFailed { hresult, error_method } => {
                println!("HResult Failed - HRESULT: {:X}, Method: {}", hresult, error_method);
            }
        }
    }
}
```

### walkdir:

```
extern crate walkdir;
use walkdir::WalkDir;
fn main() {
    for e in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            println!("{}", e.path().display());
        }
    }
}
```


### compare file old or new
```
fn is_input_file_outdated<P1, P2>(input: P1, output: P2) -> io::Result<bool>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let out_meta = fs::metadata(output);
    if let Ok(meta) = out_meta {
        let output_mtime = meta.modified()?;

        // if input file is more recent than our output, we are outdated
        let input_meta = fs::metadata(input)?;
        let input_mtime = input_meta.modified()?;

        Ok(input_mtime > output_mtime)
    } else {
        // output file not found, we are outdated
        Ok(true)
    }
}
```
