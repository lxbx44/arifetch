mod fetches;
use fetches::fetch;

use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::*;

fn main() {
    let fetch = fetch();
    let (days, hours, minutes) = fetch.uptime;

    let uptime = if hours == 0 {
        format!("{minutes} minutes")
    } else if days == 0 {
        format!("{hours} hours, {minutes} minutes")
    } else {
        format!("{days} days, {hours} hours, {minutes} minutes")
    };

    let mut table = Table::new();
    table.load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(80)
        .set_header(vec![
            String::from("\u{eb99}"),
            String::from("\u{ea9f}"),
            format!("{}@{}", fetch.username, fetch.hostname)
            ])
        .add_row(vec![
                 Cell::new("OS").add_attribute(Attribute::Bold),
                 Cell::new("\u{e712}").fg(Color::Red),
                 Cell::new(&fetch.os)
        ])
        .add_row(vec![
                 Cell::new("Kernel").add_attribute(Attribute::Bold),
                 Cell::new("\u{f109}").fg(Color::Yellow),
                 Cell::new(&fetch.kernel)
        ])
        .add_row(vec![
                 Cell::new("Uptime").add_attribute(Attribute::Bold),
                 Cell::new("\u{e385}").fg(Color::Green),
                 Cell::new(&uptime)
        ])
        .add_row(vec![
                 Cell::new("Shell").add_attribute(Attribute::Bold),
                 Cell::new("\u{f489}").fg(Color::Cyan),
                 Cell::new(&fetch.shell)
        ])
        .add_row(vec![
                 Cell::new("CPU").add_attribute(Attribute::Bold),
                 Cell::new("\u{f4bc}").fg(Color::Blue),
                 Cell::new(&fetch.cpu)
        ])
        .add_row(vec![
                 Cell::new("Memory").add_attribute(Attribute::Bold),
                 Cell::new("\u{eace}").fg(Color::Magenta),
                 Cell::new(format!("{} MiB / {} MiB", fetch.used_mem, fetch.total_mem))
        ]);

    let table = format_table(&mut table);

    println!("{table}");
}

fn format_table(table: &mut Table) -> &mut Table {
    table.set_style(TableComponent::VerticalLines, ' ');
    table.remove_style(TableComponent::HorizontalLines);
    table.remove_style(TableComponent::MiddleIntersections);
    table.set_style(TableComponent::MiddleHeaderIntersections, '─');
    table.remove_style(TableComponent::LeftBorderIntersections);
    table.remove_style(TableComponent::RightBorderIntersections);
    table.set_style(TableComponent::TopBorderIntersections, '─');
    table.set_style(TableComponent::BottomBorderIntersections, '─');
    table.set_style(TableComponent::HeaderLines, '─');
    table.set_style(TableComponent::LeftHeaderIntersection, '├');
    table.set_style(TableComponent::RightHeaderIntersection, '┤')
}
