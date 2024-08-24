use fluvio_smartmodule::{smartmodule, SmartModuleContext, Record, Result};

#[smartmodule(filter)]
pub fn my_filter(_ctx: &SmartModuleContext, record: &Record) -> Result<bool> {
    let payload = record.value.as_ref();
    Ok(payload.contains(b"desired_keyword"))
}
