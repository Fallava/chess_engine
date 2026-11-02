use std::error::Error;
use std::sync::Arc;

use futures_util::StreamExt;

use litchee::LichessClient;
use litchee::api::gameplay::board::{LichessBoardEvent, LichessGameEventInfo, LichessGameState, LichessIncomingEvent};
use litchee::api::gameplay::challenges::LichessChallenge;
use litchee::model::LichessColor;

use crate::bots::{BotTrait, BotRandom, };//BotMiniMax};
use crate::engine::{Move, Color, board::*};


pub struct Api {
    client: Arc<LichessClient>
}

impl Api {
    pub fn new() -> Self {
        let token = std::env::var("LICHESS_TOKEN").expect("Pas de token .env");
        let client = LichessClient::builder().token(token).build().expect("Token error");
        Api { client: Arc::new(client) }
    }

    pub async fn loop_event(&self) -> Result<(), Box<dyn Error>>{
        let mut stream = self.client.bot().stream_events().await?;

        while let Some(event) = stream.next().await {
            match event? {
                LichessIncomingEvent::Challenge { challenge } => {
                    if let Err(e) = self.handle_challenge(challenge).await {
                        eprintln!("Défi ignoré : {e}");
                    }
                }
                LichessIncomingEvent::GameStart { game } => {
                    if let Err(e) = self.handle_start(game).await {
                        eprintln!("Démarrage de partie échoué : {e}");
                    }
                }
                LichessIncomingEvent::GameFinish { game } => {
                    let _game = game;
                }
                LichessIncomingEvent::ChallengeCanceled { challenge } => {
                    let _challenge = challenge;
                }
                LichessIncomingEvent::ChallengeDeclined { challenge } => {
                    let _challenge = challenge;
                }
                _ => {}
            }
        }
        Ok(())
    }

    async fn handle_challenge(&self, chal: Box<LichessChallenge>) -> Result<(), Box<dyn Error>> {
        self.client.challenges().accept(&chal.id, None).await?;
        Ok(())
    }

    async fn handle_start(&self, game_info: LichessGameEventInfo) -> Result<(), Box<dyn Error>> {
        let color = match game_info.color.unwrap_or(LichessColor::White) {
            LichessColor::White => Color::White,
            LichessColor::Black => Color::Black
        };

        let client_clone = Arc::clone(&self.client);
        let Some(id) = game_info.id else {
            eprintln!("GameStart sans identifiant");
            return Ok(());
        };

        tokio::spawn(async move {
            let mut game = Game::new(client_clone, id, color, BotRandom);

            game.main_loop().await.unwrap_or_else(|e| {
                eprintln!("Erreur critique dans la partie {}: {}", game.id, e);
            });
        });
        Ok(())
    }
}


#[derive(Debug)]
struct Game<B: BotTrait> {
    client: Arc<LichessClient>,
    id: String,
    board: Board,
    color: Color,
    board_move: usize,
    sent_move: Option<usize>,
    bot: B
}

impl <B: BotTrait> Game<B>{
    fn new(client: Arc<LichessClient>, id: String, color: Color, bot: B) -> Self {
        Game {
            client,
            id,
            board: Board::startpos(),
            color,
            board_move: 0,
            sent_move: None,
            bot
        }
    }

    async fn main_loop(&mut self) -> Result<(), Box<dyn Error>> {
        let mut game_stream = self.client.bot().stream_game(&self.id).await?;

        while let Some(game_event) = game_stream.next().await {
            match game_event? {
                LichessBoardEvent::GameFull(game_full) => {
                    self.check_state(game_full.state).await?;
                }
                LichessBoardEvent::GameState(state) => {
                    self.check_state(state).await?;
                }
                LichessBoardEvent::OpponentGone(gone) => {
                    println!("{:#?}", gone);
                }
                LichessBoardEvent::ChatLine(chat) => {
                    println!("{:#?}", chat);
                }
                _ => {}
            }
        }
        Ok(())
    } 

    async fn check_state(&mut self, state: LichessGameState) -> Result<(), Box<dyn Error>>{
        let movs: Vec<&str> = state.moves.split_whitespace().collect();

        for m in &movs[self.board_move..] {
            let mov_to_play = Move::from_uci(m).ok_or_else(|| format!("coup illisible : {m}"))?;
            self.board.make_move(mov_to_play);
        }

        self.board_move = movs.len();

        if self.color == self.board.turn && self.sent_move != Some(self.board_move) {
            if let Some(mv) = self.bot.play_move(&self.board, &self.id) {
                match self.play_mov(mv).await {
                    Ok(()) => self.sent_move = Some(self.board_move),
                    Err(e) => eprintln!("Partie {} : coup refusé : {e}", self.id),
                }
            }
        }
        Ok(())
    }

    async fn play_mov(&self,mov: Move) -> Result<(), Box<dyn Error>> {
        let mov = mov.to_uci();

        self.client.bot().make_move(&self.id, &mov, false).await?;
        Ok(())
    }    
}
