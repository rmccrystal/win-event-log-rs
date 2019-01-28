extern crate win_event_log;

use win_event_log::{Comparison, Condition, EventFilter, Query, QueryList, Selector, WinEvents};

fn main() {
    let conditions = vec![
        Condition::filter(EventFilter::level(1, Comparison::Equal)),
        Condition::filter(EventFilter::level(4, Comparison::GreaterThanOrEqual)),
    ];
    let query = QueryList::new()
        .with_query(
            Query::new()
                .select(
                    Selector::new("Application".to_owned())
                        .system_conditions(Condition::or(conditions))
                        .build(),
                )
                .query(),
        )
        .build();

    match WinEvents::get(query) {
        Ok(events) => {
            if let Some(event) = events.into_iter().next() {
                println!("{}", event);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
