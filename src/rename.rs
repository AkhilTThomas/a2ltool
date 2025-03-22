use a2lfile::A2lFile;
// match regex and rename the items

pub(crate) fn rename_items(
    a2l_file: &mut A2lFile,
    prefix_strings: &[&str],
    log_messages: &mut Vec<String>,
) -> usize {
    let mut renamed_items = std::collections::HashSet::<String>::new();

    for module in &mut a2l_file.project.module {
        // Rename all characteristics that match any of the prefixes
        module.characteristic = module
            .characteristic
            .drain(..)
            .map(|mut characteristic| {
                for prefix in prefix_strings {
                    if let Some(renamed) = characteristic.name.strip_prefix(prefix) {
                        renamed_items.insert(characteristic.name.clone());
                        log_messages.push(format!(
                            "Renamed characteristic {} to {}",
                            characteristic.name, renamed
                        ));
                        characteristic.name.replace_range(..prefix.len(), "");
                    }
                }
                characteristic
            })
            .collect();

        // Rename all measurements that match any of the prefixes
        module.measurement = module
            .measurement
            .drain(..)
            .map(|mut measurement| {
                for prefix in prefix_strings {
                    if let Some(renamed) = measurement.name.strip_prefix(prefix) {
                        renamed_items.insert(measurement.name.clone());
                        log_messages.push(format!(
                            "Renamed measurement {} to {}",
                            measurement.name, renamed
                        ));
                        measurement.name.replace_range(..prefix.len(), "");
                    }
                }
                measurement
            })
            .collect();
    }
    renamed_items.len()
}
