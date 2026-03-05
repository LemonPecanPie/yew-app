use yew::prelude::*;

// comment added for commit

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
}

#[component]
fn VideosList(VideosListProps { videos }: &VideosListProps) -> Html {
    html! {
        for video in videos {
            <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
        }
    }
}

#[derive(Clone, PartialEq)]
struct Video {
    id: usize,
    title: AttrValue,
    speaker: AttrValue,
    url: AttrValue,
}

#[component]
fn App() -> Html {
    let videos = vec![
        Video {
            id: 1,
            title: "Building and breaking things".into(),
            speaker: "John Doe".into(),
            url: "https://youtu.be/PsaFVLr8t4E".into(),
        },
        Video {
            id: 2,
            title: "The development process".into(),
            speaker: "Jane Smith".into(),
            url: "https://youtu.be/PsaFVLr8t4E".into(),
        },
        Video {
            id: 3,
            title: "The Web 7.0".into(),
            speaker: "Matt Miller".into(),
            url: "https://youtu.be/PsaFVLr8t4E".into(),
        },
        Video {
            id: 4,
            title: "Mouseless development".into(),
            speaker: "Tom Jerry".into(),
            url: "https://youtu.be/PsaFVLr8t4E".into(),
        },
    ];

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{ "Videos to watch" }</h3>
                <VideosList {videos} />
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
