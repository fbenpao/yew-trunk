use yew::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
// use js_sys::Promise;
use js_sys::Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, JsFuture, spawn_local};
use web_sys::{AudioContext, Navigator, MediaDevices, MediaRecorder, MediaStreamConstraints, MediaStream,Url,RecordingState,Blob};
use web_sys::console as yew_console;
use yew::{
    Component,
     Html,
     html, 
     Context,
};

pub struct Model {
    recorder:FetchState<MediaRecorder>,
    chunk:Array,
    audio_path:String
}

pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(JsValue),
}

pub enum Msg {
    RecorderInstance(FetchState<MediaRecorder>),
	StartRecorder,
    PauseRecorder,
	StopRecorder,
	GetRecorderData,
    None
}

pub async fn get_media()-> Result<MediaStream, JsValue> {
	//打开麦克风录音
	let window = web_sys::window().expect("Missing Window");
	let navigator = window.navigator();
	let media_device = navigator.media_devices().unwrap();
	let mut media_constraints = web_sys::MediaStreamConstraints::new();
	media_constraints.audio(&JsValue::TRUE);
	media_constraints.video(&JsValue::FALSE);
	let promise  = media_device.get_user_media_with_constraints(&media_constraints).unwrap();
	let user_media = JsFuture::from(promise).await?.dyn_into::<web_sys::MediaStream>()?;
	Ok(user_media)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // let blob = Blob::new().unwrap();
        let init = Self {
            recorder: FetchState::NotFetching,
            chunk: Array::new(),
            audio_path:String::from("https://sharefs.ali.kugou.com/202111302207/17b015a4f36bf653378336990509391c/G248/M00/11/10/2JQEAF-_cTKAV49VAD4tghc2xQU534.mp3")
            // audio_path:Url::create_object_url_with_blob(&blob).unwrap()
        };
        ctx.link().send_future(async {
            let media_stream = get_media().await.unwrap();
            let media_recorder = web_sys::MediaRecorder::new_with_media_stream(&media_stream);
            match media_recorder {
                Ok(rc) => Msg::RecorderInstance(FetchState::Success(rc)),
                Err(err) => Msg::RecorderInstance(FetchState::Failed(err)),
            }
        });
        ctx.link()
            .send_message(Msg::RecorderInstance(FetchState::Fetching));

        init
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            Msg::RecorderInstance(fetch_state) => {
                self.recorder = fetch_state;
                true
            }
            Msg::StartRecorder =>{
                yew_console::log_1(&"录音".into());
                // ctx.link().send_future(async {
                //     let media_stream = get_media().await.unwrap();
				// 	let media_recorder = web_sys::MediaRecorder::new_with_media_stream(&media_stream);
                //     match media_recorder {
                //         Ok(rc) => Msg::RecorderInstance(FetchState::Success(rc)),
                //         Err(err) => Msg::RecorderInstance(FetchState::Failed(err)),
                //     }
                // });
                // ctx.link()
                //     .send_message(Msg::RecorderInstance(FetchState::Fetching));
                match &self.recorder{
                    FetchState::Success(rc) =>{
                        rc.start();
                        // self.chunk.push(&1.into());
                        if let Some(e) = rc.ondataavailable(){
                            self.chunk.push(&e.into());
                        }
                        // match rc.ondataavailable(){
                          
                        // } 
                        let state = rc.state();
                        yew_console::log_1(&format!("{:?}", state).into());
                        yew_console::log_1(&"录音 ing".into());

                    }
                    FetchState::Failed(err) =>{
                        yew_console::log_1(err);

                    }

                    FetchState::Fetching =>{
                        yew_console::log_1(&"fetching".into());

                    }
                    FetchState::NotFetching =>{
                        yew_console::log_1(&"not fetching".into());

                    }
                }
                true
            }

            Msg::PauseRecorder => {
                yew_console::log_1(&"暂停录音".into());
                yew_console::log_1(&self.chunk);
                match &self.recorder{
                    FetchState::Success(rc) =>{
                        rc.pause();
                        let state = rc.state();
                        yew_console::log_1(&format!("{:?}", state).into());
                        yew_console::log_1(&"录音 pause".into());

                    }
                    FetchState::Failed(err) =>{
                        yew_console::log_1(err);

                    }

                    FetchState::Fetching =>{
                        yew_console::log_1(&"fetching".into());

                    }
                    FetchState::NotFetching =>{
                        yew_console::log_1(&"not fetching".into());

                    }
                }

                true

            }
                
            
            Msg::StopRecorder =>{
                yew_console::log_1(&"结束录音".into());
                match &self.recorder{
                    FetchState::Success(rc) =>{
                        rc.stop();
                        if let Some(_e) = rc.onstop(){
                            yew_console::log_1(&"onstop".into());
                            let blob = Blob::new_with_buffer_source_sequence(&self.chunk).unwrap();
                            let audio_url = Url::create_object_url_with_blob(&blob).unwrap();
                            self.audio_path = audio_url;
                        }
                        let state = rc.state();
                        yew_console::log_1(&format!("{:?}", state).into());
                        yew_console::log_1(&"录音 end".into());

                    }
                    FetchState::Failed(err) =>{
                        yew_console::log_1(err);

                    }

                    FetchState::Fetching =>{
                        yew_console::log_1(&"fetching".into());

                    }
                    FetchState::NotFetching =>{
                        yew_console::log_1(&"not fetching".into());

                    }
                }
                true
            }
            Msg::GetRecorderData =>{
                true
            }
            Msg::None => {
                false
            }

        }
    }



    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <main>
                <h1>{ "video diy" }</h1>
                <div> <audio  src={self.audio_path.clone()} controls=true class="audio-player" /></div>
                <div> <button  onclick={link.callback(|_| Msg::StartRecorder)} >{"录音"}</button></div>
                <div> <button  onclick={link.callback(|_| Msg::PauseRecorder)} >{"暂停"}</button></div>
                <div> <button  onclick={link.callback(|_| Msg::StopRecorder)}>{"结束"}</button></div>
                <div> <button  onclick={link.callback(|_| Msg::GetRecorderData)}>{"播放录音"}</button></div>
            </main>
        }

    }
}

fn main() {
    yew::start_app::<Model>();
}
