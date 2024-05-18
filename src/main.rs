use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct ParkingSpot {
    id: usize,
    occupied: bool,
    reserved: bool,
}

impl ParkingSpot {
    fn new(id: usize) -> Self {
        Self {
            id,
            occupied: false,
            reserved: false,
        }
    }
}

struct ParkingLot {
    spots: Vec<ParkingSpot>,
    reservations: HashMap<usize, String>, // Map of spot ID to reservation details
}

impl ParkingLot {
    fn new(size: usize) -> Self {
        let mut spots = Vec::with_capacity(size);
        for i in 0..size {
            spots.push(ParkingSpot::new(i));
        }
        Self {
            spots,
            reservations: HashMap::new(),
        }
    }

    fn find_available_spot(&self) -> Option<&ParkingSpot> {
        self.spots.iter().find(|spot| !spot.occupied && !spot.reserved)
    }

    fn find_nearest_available_spot(&self, position: usize) -> Option<&ParkingSpot> {
        let mut nearest_spot: Option<&ParkingSpot> = None;
        let mut min_distance = usize::MAX;

        for spot in &self.spots {
            if !spot.occupied && !spot.reserved {
                let distance = if spot.id >= position {
                    spot.id - position
                } else {
                    position - spot.id
                };

                if distance < min_distance {
                    min_distance = distance;
                    nearest_spot = Some(spot);
                }
            }
        }
        nearest_spot
    }

    fn park_car(&mut self) -> Result<usize, &'static str> {
        if let Some(spot) = self.spots.iter_mut().find(|spot| !spot.occupied && !spot.reserved) {
            spot.occupied = true;
            Ok(spot.id)
        } else {
            Err("No available spots")
        }
    }

    fn park_car_in_spot(&mut self, id: usize) -> Result<(), &'static str> {
        if id >= self.spots.len() {
            return Err("Invalid spot ID");
        }
        let spot = &mut self.spots[id];
        if spot.occupied || spot.reserved {
            return Err("Spot already occupied or reserved");
        }
        spot.occupied = true;
        Ok(())
    }

    fn remove_car(&mut self, id: usize) -> Result<(), &'static str> {
        if let Some(spot) = self.spots.iter_mut().find(|spot| spot.id == id && spot.occupied) {
            spot.occupied = false;
            Ok(())
        } else {
            Err("Spot not found or already empty")
        }
    }

    fn list_spots(&self) {
        for spot in &self.spots {
            let status = if spot.occupied {
                "Occupied"
            } else if spot.reserved {
                "Reserved"
            } else {
                "Available"
            };
            println!("Spot {}: {}", spot.id, status);
        }
    }

    fn reserve_spot(&mut self, id: usize, details: String) -> Result<(), &'static str> {
        if id >= self.spots.len() {
            return Err("Invalid spot ID");
        }
        let spot = &mut self.spots[id];
        if spot.occupied || spot.reserved {
            return Err("Spot already occupied or reserved");
        }
        spot.reserved = true;
        self.reservations.insert(id, details);
        Ok(())
    }

    fn cancel_reservation(&mut self, id: usize) -> Result<(), &'static str> {
        if id >= self.spots.len() || !self.spots[id].reserved {
            return Err("Invalid spot ID or spot not reserved");
        }
        self.spots[id].reserved = false;
        self.reservations.remove(&id);
        Ok(())
    }
}

fn display_help() {
    println!("Parking Lot Help:");
    println!("1. Park car in next available spot: Automatically parks your car in the next available spot.");
    println!("2. Park car in specific spot: Allows you to choose a specific spot to park your car.");
    println!("3. Remove car from spot: Removes the car from the specified spot.");
    println!("4. List all spots: Displays the status of all parking spots (Occupied, Reserved, or Available).");
    println!("5. Find nearest available spot: Finds the nearest available spot from your current position.");
    println!("6. Reserve a spot in advance: Allows you to reserve a parking spot for future use.");
    println!("7. Cancel a reservation: Cancels an existing reservation for a spot.");
    println!("8. Exit: Exits the parking lot system.");
    println!("9. Help: Displays this help information.");
}

fn main() {
    let mut parking_lot = ParkingLot::new(10);

    loop {
        println!("\nParking Lot Menu:");
        println!("1. Park car in next available spot");
        println!("2. Park car in specific spot");
        println!("3. Remove car from spot");
        println!("4. List all spots");
        println!("5. Find nearest available spot");
        println!("6. Reserve a spot in advance");
        println!("7. Cancel a reservation");
        println!("8. Exit");
        println!("9. Help");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                match parking_lot.park_car() {
                    Ok(id) => println!("Car parked in spot {}", id),
                    Err(err) => println!("Error: {}", err),
                }
            }
            2 => {
                print!("Enter the spot number where you want to park the car: ");
                io::stdout().flush().unwrap();
                let mut spot = String::new();
                io::stdin().read_line(&mut spot).unwrap();
                let spot: usize = match spot.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid spot number.");
                        continue;
                    },
                };

                match parking_lot.park_car_in_spot(spot) {
                    Ok(_) => println!("Car parked in spot {}", spot),
                    Err(err) => println!("Error: {}", err),
                }
            }
            3 => {
                print!("Enter the spot number to remove the car from: ");
                io::stdout().flush().unwrap();
                let mut spot = String::new();
                io::stdin().read_line(&mut spot).unwrap();
                let spot: usize = match spot.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid spot number.");
                        continue;
                    },
                };

                match parking_lot.remove_car(spot) {
                    Ok(_) => println!("Car removed from spot {}", spot),
                    Err(err) => println!("Error: {}", err),
                }
            }
            4 => {
                println!("Parking lot status:");
                parking_lot.list_spots();
            }
            5 => {
                print!("Enter your current position: ");
                io::stdout().flush().unwrap();
                let mut position = String::new();
                io::stdin().read_line(&mut position).unwrap();
                let position: usize = match position.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid position.");
                        continue;
                    },
                };

                match parking_lot.find_nearest_available_spot(position) {
                    Some(spot) => println!("Nearest available spot is {}", spot.id),
                    None => println!("No available spots"),
                }
            }
            6 => {
                print!("Enter the spot number to reserve: ");
                io::stdout().flush().unwrap();
                let mut spot = String::new();
                io::stdin().read_line(&mut spot).unwrap();
                let spot: usize = match spot.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid spot number.");
                        continue;
                    },
                };

                print!("Enter reservation details: ");
                io::stdout().flush().unwrap();
                let mut details = String::new();
                io::stdin().read_line(&mut details).unwrap();
                let details = details.trim().to_string();

                match parking_lot.reserve_spot(spot, details) {
                    Ok(_) => println!("Spot {} reserved", spot),
                    Err(err) => println!("Error: {}", err),
                }
            }
            7 => {
                print!("Enter the spot number to cancel the reservation: ");
                io::stdout().flush().unwrap();
                let mut spot = String::new();
                io::stdin().read_line(&mut spot).unwrap();
                let spot: usize = match spot.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid spot number.");
                        continue;
                    },
                };

                match parking_lot.cancel_reservation(spot) {
                    Ok(_) => println!("Reservation for spot {} canceled", spot),
                    Err(err) => println!("Error: {}", err),
                }
            }
            8 => {
                println!("Exiting...");
                break;
            }
            9 => {
                display_help();
            }
            _ => {
                println!("Invalid choice. Please choose a valid option.");
            }
        }
    }
}