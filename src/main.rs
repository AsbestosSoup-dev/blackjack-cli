mod models;
mod ui;
// struct PlayerQueue {
//     waitlist: Vec<Player>,
// }



fn main() {
    // Demo
    println!("WELCOME TO BLACKJACK CLI");
    let card1 = models::card::Card(models::card::Rank::Ace, models::card::Suit::Hearts);
    let card2 = models::card::Card(models::card::Rank::Ace, models::card::Suit::Diamonds);
    let card3 = models::card::Card(models::card::Rank::Ace, models::card::Suit::Clubs);
    let card4 = models::card::Card(models::card::Rank::Ace, models::card::Suit::Spades);
    
    println!("{card1}, {card2}, {card3}, {card4}");
    
    // for i in 0..num_players {
    //     let mut player = Player {
    //         uuid: Uuid::new_v4(),
    //         name: format!("Player {}", i + 1),
    //         hands: vec![Hand {
    //             cards: Vec::new(),
    //             bet: 0,
    //             is_split: false,
    //             is_doubled: false,
    //             is_bust: false,
    //             score: None,
    //         }],
    //         bank: 1000,
    //         score: 0,
    //     };
    //     println!("Created player {} ({})", player.name, player.uuid);
    //     game_state.players.push(player);
    // }



    // print!("How many decks in play? (max 8) ");
    // stdout().flush().unwrap();
    // let mut input = String::new();
    // stdin().read_line(&mut input).unwrap();
    // let num_decks: u8 = input.trim().parse().unwrap();
    // 
    // if !(1..=8).contains(&num_decks) {
    //     println!("Invalid number of decks ({num_decks}). Must be between 1 and 8.");
    //     return;
    // }
    // 
    // game_state.deck = Deck::new(num_decks);
    // game_state.deck.cut_position = u16::from(num_decks) * 52;
    // 
    // let mut rng = rand::rng();
    // println!("Shuffling deck...");
    // game_state.deck.cards.shuffle(&mut rng);
    // let cut_position_low = (f32::from(game_state.deck.cut_position) * 0.7f32) as u16;
    // let cut_position_high = (f32::from(game_state.deck.cut_position) * 0.75f32) as u16;
    // game_state.deck.cut_position = rand::random_range(cut_position_low ..= cut_position_high);
    // println!("Deck cut position set to {} of {}", game_state.deck.cut_position, game_state.deck.cards.len() - 1);
    // println!("Shuffling done.");
    // 
    // println!("Starting game...");
    // game_state.phase = GamePhase::Betting;
    // for player in &mut game_state.players {
    //     assert_eq!(player.hands.len(), 1);
    //     player.hands[0].bet = 100;
    //     player.bank -= player.hands[0].bet;
    //     println!("Player {} bet ${}", player.name, player.hands[0].bet);
    // }
    // game_state.phase = GamePhase::Dealing;
    // println!("First dealing:");
    // for i in 0..num_players {
    //     let i = i as usize;
    //     let drawn_card = &game_state.deck.cards[game_state.deck.current_position as usize];
    //     game_state.players[i].hands[0].cards.push(drawn_card);
    //     game_state.deck.current_position += 1;
    //     println!("{} draws {:?}", game_state.players[i].name, drawn_card);
    // }
    // let first_dealer_card = &game_state.deck.cards[game_state.deck.current_position as usize];
    // game_state.dealer.hand.push(first_dealer_card);
    // game_state.deck.current_position += 1;
    // let mut dealer_got_ace = false;
    // let mut dealer_got_10 = false;
    // if let Card(Rank::Ace, _) = first_dealer_card {
    //     dealer_got_ace = true;
    // } else if let Card(Rank::Ten, _) = first_dealer_card {
    //     dealer_got_10 = true;
    // }
    // println!("Dealer draws {first_dealer_card:?}\n");
    // 
    // println!("Second dealing:\n");
    // 
    // for i in 0..num_players {
    //     let i = i as usize;
    //     let drawn_card = &game_state.deck.cards[game_state.deck.current_position as usize];
    //     game_state.players[i].hands[0].cards.push(drawn_card);
    //     game_state.deck.current_position += 1;
    //     println!("{} draws {:?}", game_state.players[i].name, drawn_card);
    // }
    // 
    // let second_dealer_card = &game_state.deck.cards[game_state.deck.current_position as usize];
    // game_state.dealer.hand.push(second_dealer_card);
    // game_state.deck.current_position += 1;
    // if let Card(Rank::Ace, _) = second_dealer_card {
    //     dealer_got_ace = true;
    // } else if let Card(Rank::Ten, _) = second_dealer_card {
    //     dealer_got_10 = true;
    // }
    // 
    // let mut dealer_got_blackjack = false;
    // 
    // if dealer_got_ace && dealer_got_10 {
    //     println!("Dealer drew {second_dealer_card:?}!\n");
    //     println!("DEALER BLACKJACK!");
    //     dealer_got_blackjack = true;
    // }
    // 
    // game_state.phase = GamePhase::PlayerTurns;
    // 
    // for player in &mut game_state.players {
    //     println!("{} what will you do?", player.name);
    //     if player.hands[0].cards[0].0 == player.hands[0].cards[1].0 {
    //         print!("[w] hit | [a] double down | [s] stand | [d] split: ");    
    //     } else {
    //         print!("[w] hit | [a] double down | [s] stand: ");    
    //     }
    //     stdout().flush().unwrap();
    //     let mut input = String::new();
    //     stdin().read_line(&mut input).unwrap();
    //     match input.trim() {
    //         "w" => println!("{} hits card", player.name),
    //         "a" => println!("{} doubled down", player.name),
    //         "s" => println!("{} stands card", player.name),
    //         "d" => println!("{} splits card", player.name),
    //         _ => println!("Invalid input!"),
    //     }
    // }

}
