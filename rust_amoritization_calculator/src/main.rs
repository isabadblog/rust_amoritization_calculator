use std::io::{self, Write};

/// Calculates the monthly payment for an amortizing loan.
fn calculate_monthly_payment(principal: f64, annual_rate: f64, years: u32) -> f64 {
    let monthly_rate = (annual_rate / 100.0) / 12.0;
    let number_of_payments = (years * 12) as f64;

    if monthly_rate == 0.0 {
        return principal / number_of_payments;
    }

    principal * (monthly_rate * (1.0 + monthly_rate).powf(number_of_payments))
        / ((1.0 + monthly_rate).powf(number_of_payments) - 1.0)
}

/// Generates and prints the amortization schedule.
fn print_schedule(principal: f64, annual_rate: f64, years: u32) {
    let monthly_rate = (annual_rate / 100.0) / 12.0;
    let number_of_payments = years * 12;
    let monthly_payment = calculate_monthly_payment(principal, annual_rate, years);

    println!("\nAmortization Schedule\n");
    println!(
        "{:<5} {:<15} {:<15} {:<15} {:<15}",
        "Month", "Payment", "Principal", "Interest", "Balance"
    );

    let mut remaining_balance = principal;

    for month in 1..=number_of_payments {
        let interest_payment = remaining_balance * monthly_rate;
        let principal_payment = monthly_payment - interest_payment;
        remaining_balance -= principal_payment;

        println!(
            "{:<5} ${:<14.2} ${:<14.2} ${:<14.2} ${:<14.2}",
            month,
            monthly_payment,
            principal_payment,
            interest_payment,
            remaining_balance.max(0.0) // Ensure balance doesn't go negative
        );
    }
}
fn get_user_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        eprint!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }
}
fn main() {
    eprintln!("Loan Amortization Calculator");
    eprintln!("----------------------------");

    // Get loan details from user
    let principal = get_user_input("Enter the loan amount (e.g., 200000): ");
    let annual_rate = get_user_input("Enter the annual interest rate (e.g., 3.5): ");
    let years: u32 = get_user_input("Enter the loan term in years (e.g., 30): ");

    // Calculate and display results
    let monthly_payment = calculate_monthly_payment(principal, annual_rate, years);
    println!("\nCalculated Monthly Payment: ${:.2}\n", monthly_payment);

    // Print the full amortization schedule
    print_schedule(principal, annual_rate, years);
}