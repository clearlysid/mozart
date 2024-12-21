let audioContextReady = false;

const listen = window.__TAURI__.event.listen;

const startButton = document.getElementById("start");

startButton.onclick = async () => {
  await Tone.start();
}

// create a synth and connect it to the main output (your speakers)
const sampler = new Tone.Sampler({
    urls: {
        A1: "A1.mp3",
        A2: "A2.mp3",
    },
    baseUrl: "https://tonejs.github.io/audio/casio/",
}).toDestination();

listen('key-pressed', (event) => {
    // What notes pick from?
    const bluesNotes = ["C2", "D#2", "F2", "F#2", "G2", "A#2", "C3", "D#3", "F3", "F#3", "G3", "A#3"];

    // Pick a random note
    const randomNote = bluesNotes[Math.floor(Math.random() * bluesNotes.length)];

    // Generate a random note
    const randomVelocity = Math.random() * 0.7 + 0.3; // Random velocity between 0.3 and 1.0

    // Play the note
    sampler.triggerAttackRelease(randomNote, 0.3, undefined, randomVelocity);
});