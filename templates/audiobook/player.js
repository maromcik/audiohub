let vid = document.getElementById("audiobook-player");

function getCurTime() {
    alert(vid.currentTime);
}

function setCurTime(t) {
    vid.currentTime = t;
}

