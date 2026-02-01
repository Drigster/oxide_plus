use freya::{
    prelude::*,
    radio::{RadioStation, use_share_radio},
    router::prelude::{Routable, RouterConfig},
};
use freya_router::prelude::Router;

use crate::layouts::{LoginLayout, MainLayout, MapLayout, RouteChangeRecieverLayout};
use crate::pages::{Info, Loading, Login, Map, MinimapSettingsPage, ServerSelect, Shops, Team};
use crate::{Data, DataChannel};

pub struct App {
    pub radio_station: RadioStation<Data, DataChannel>,
}

impl Component for App {
    fn render(&self) -> impl IntoElement {
        use_share_radio(move || self.radio_station);

        Router::new(|| RouterConfig::<Route>::default().with_initial_path(Route::Loading))
    }
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(RouteChangeRecieverLayout)]
        #[route("/login")]
        Login,
        #[layout(LoginLayout)]
            #[route("/")]
            Loading,
            #[route("/select_server")]
            ServerSelect,
        #[end_layout]
        #[layout(MainLayout)]
            #[route("/info")]
            Info,
            #[nest("/map")]
                #[layout(MapLayout)]
                    #[route("/")]
                    Map,
                    #[route("/minimap_settings")]
                    MinimapSettingsPage,
                #[end_layout]
            #[end_nest]
        #[route("/team")]
        Team,
        #[route("/shops")]
        Shops,
}
