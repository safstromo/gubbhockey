use leptos::{logging::log, prelude::*};
use serde::{Deserialize, Serialize};

#[component]
pub fn DatePicker() -> impl IntoView {
    let submit = ServerAction::<AddDate>::new();
    view! {
        <ActionForm action=submit>
            <div class="flex flex-col m-2">
                <label for="input_date[date]" class="">
                    Datum
                </label>
                <input type="date" name="input_date[date]" />
                <label for="input_date[start]" class="">
                    Start
                </label>
                <input type="time" name="input_date[start]" />
                <label for="input_date[end]" class="">
                    Slut
                </label>
                <input type="time" name="input_date[end]" />
            </div>
            <button class="btn" type="submit">
                Lägg till dag
            </button>

        </ActionForm>
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InputDate {
    date: String,
    start: String,
    end: String,
}

#[server]
async fn add_date(input_date: InputDate) -> Result<(), ServerFnError> {
    log!("Date submit: {:?}", input_date);
    Ok(())
}
