use online::check;
use rodio::{Decoder, OutputStreamBuilder, Sink, source::SineWave, Source};
use std::{fs::File, io::BufReader, time::Duration, thread};
use tokio::{task, time};

/// Reproduz `alert.wav` sem bloquear a tarefa principal.

fn play_direct_alert() -> Result<(), Box<dyn std::error::Error>> {
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let mixer = stream_handle.mixer();

    let beep1 = {
        // Play a WAV file.
        let file = std::fs::File::open("alert.mp3")?;
        let sink = rodio::play(mixer, BufReader::new(file))?;
        sink.set_volume(0.2);
        sink
    };

    println!("Started beep1");
    thread::sleep(Duration::from_millis(1500));

    {
        // Generate sine wave.
        let wave = SineWave::new(740.0)
            .amplify(0.2)
            .take_duration(Duration::from_secs(3));
        mixer.add(wave);
    }

    Ok(())
}
fn play_alert() {
    let stream_handle = match OutputStreamBuilder::open_default_stream() {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Falha ao abrir áudio: {e}");
            return;
        }
    };
    let sink = Sink::connect_new(&stream_handle.mixer());
    sink.set_volume(1.0);
    if let Ok(file) = File::open("alert.mp3") {
        if let Ok(src) = Decoder::new(BufReader::new(file)) {
            sink.append(src);
            sink.detach();          // toca em background
        }
    } else {
        eprintln!("Erro abrindo alert.mp3");
    }
}

#[tokio::main]
async fn main() {
    const INTERVAL: Duration = Duration::from_secs(10);
    let mut wifi_ok = true;

    loop {
        let ok_now = task::spawn_blocking(|| check(None))
            .await
            .is_ok();

        // dispara quando mudar de online → offline
        if wifi_ok && ok_now {
            eprintln!("⚠️  Conexão Wi-Fi/Internet perdida!");
            play_direct_alert().expect("TODO: panic message");
        }

        wifi_ok = ok_now;
        time::sleep(INTERVAL).await;
    }
}