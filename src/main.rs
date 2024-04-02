slint::include_modules!();

const TAX_PERCENTAGE: f64 = 0.30;
const OWNERSHIP_PERCENTAGE: f64 = 0.55;
const PROFIT_PERCENTAGE: f64 = 0.05;
const OPERATING_PERCENTAGE: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_divide_income(move | amount | {
        let ui = ui_handle.unwrap();
        let num: f64 = amount.trim().parse().unwrap();

        let tax = num * TAX_PERCENTAGE;
        let ownership = num * OWNERSHIP_PERCENTAGE;
        let profit = num * PROFIT_PERCENTAGE;
        let operating = num * OPERATING_PERCENTAGE;

        let result = format!("Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOperating Expenses: {:.2}", tax, ownership, profit, operating);
        ui.set_results(result.into()); 

    });

    ui.run()
}
