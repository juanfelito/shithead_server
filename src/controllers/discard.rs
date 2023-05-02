use crate::shithead::discard_server::{Discard};
use crate::shithead::{GetDiscardRequest, GetDiscardResponse};
use tonic::{Request, Response, Status};
use crate::mediators::discard::DiscardMediator;

#[derive(Debug)]
pub struct DiscardService {
    mediator: DiscardMediator
}

impl DiscardService {
    pub fn new(mediator: DiscardMediator) -> Self {
        DiscardService { mediator }
    }
}

#[tonic::async_trait]
impl Discard for DiscardService {
    async fn get_discard(
        &self,
        request: Request<GetDiscardRequest>
    ) -> Result<Response<GetDiscardResponse>, Status> {
        println!("Got a get request: {:?}", request);

        let req = request.into_inner();

        let res = self.mediator.get_discard(req.game_id).await;
        match res {
            Ok(discard) => {
                let reply = GetDiscardResponse { 
                    current_value: discard.inner.current_value,
                    current_card: discard.inner.current_card,
                    id: discard.id.id.to_string(),
                    repeat_count: discard.inner.repeat_count,
                    cards: discard.inner.cards,
                };
                Ok(Response::new(reply))
            }
            Err(err) => {
                println!("{:?}", err);
                Err(Status::not_found("couldn't find the requested game"))
            }
        }
    }
}
