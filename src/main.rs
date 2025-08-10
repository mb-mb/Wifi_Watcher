use rodio::{Decoder, OutputStreamBuilder, Sink, source::SineWave, Source};
use std::{fs::File, io::BufReader, time::Duration, thread};

fn play_direct_alert(sink: &Sink) {
    // Toca arquivo
    let file = File::open("alert.mp3").unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap().amplify(0.2);
    sink.append(source);

    thread::sleep(Duration::from_millis(1500));

    // Toca onda senoidal
    let wave = SineWave::new(740.0)
        .amplify(0.2)
        .take_duration(Duration::from_secs(3));
    sink.append(wave);
}

#[tokio::main]
async fn main() {
    const INTERVAL: Duration = Duration::from_secs(2);
    let mut wifi_ok = true;

    // No 0.21.1, criamos o stream e pegamos o mixer separadamente
    let stream = OutputStreamBuilder::open_default_stream().unwrap();
    let mixer = stream.mixer();
    let sink = Sink::connect_new(&mixer);

    loop {
        let ok_now = online::check(None).is_ok();

        if wifi_ok && !ok_now {
            eprintln!("⚠️  Conexão Wi-Fi/Internet perdida!");
            play_direct_alert(&sink);
        } else if !wifi_ok && ok_now {
            eprintln!("✅ Conexão restaurada");
        } else if ok_now {
            eprintln!("Wi-Fi conectado com sucesso");
        }

        wifi_ok = ok_now;
        tokio::time::sleep(INTERVAL).await;
    }
}
