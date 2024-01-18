const setCurTime = (time) => {
    console.log(time)
    const audio = document.getElementById("audiobook-player");
    audio.currentTime = time;
}