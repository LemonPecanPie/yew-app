use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;

// comment added for commit
// another comment added for commit
// yet another commend added for commit

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[component]
fn VideoDetails(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
        <h3>{ &*video.title }</h3>
        <img src="https://placehold.co/640x360.png?text=Video+Player+Placeholder"
        alt="video thumbnail" />
        </div>
    }
}

#[component]
fn VideosList(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_select = |video: &Video| {
        let on_click = on_click.clone();
        let video = video.clone();
        Callback::from(move |_| {
            on_click.emit(video.clone());
        })
    };
    html! {
        for video in videos {
            <p key={video.id} onclick={on_select(video)}>{format!("{}: {}", video.speaker, video.title)}</p>
        }
    }
}

#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: AttrValue,
    speaker: AttrValue,
    url: AttrValue,
}

#[component]
fn App() -> Html {
    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with((), move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> =
                Request::get("/tutorial/data.json")
                .send().await.unwrap().json().await.unwrap();
            videos.set(fetched_videos);
            });
            || ()
        });

    }

    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video));
        })
    };

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{ "Videos to watch" }</h3>
                <VideosList videos={(*videos).clone()} on_click={on_video_select} />
            </div>
            if let Some(video) = &*selected_video {
                <VideoDetails video={video.clone()} />
            }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
