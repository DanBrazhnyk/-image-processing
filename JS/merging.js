let originalCanvas = document.getElementById("merge1Canvas");
let originalCtx = originalCanvas.getContext("2d");

let original2Canvas = document.getElementById("merge2Canvas");
let original2Ctx = original2Canvas.getContext("2d");

let mergingCanvas = document.getElementById("mergingCanvas");
let mergingCtx = mergingCanvas.getContext("2d");

let image1 = new Image();
image1.onload = function () {
  originalCanvas.width = 300;
  originalCanvas.height = 300;
  originalCtx.drawImage(image1, 0, 0, 300, 300);
};

let image2 = new Image();
image2.onload = function () {
  original2Canvas.width = 300;
  original2Canvas.height = 300;
  original2Ctx.drawImage(image2, 0, 0, 300, 300);
};
document.getElementById("fileInput").addEventListener("change", function (e) {
  let file = e.target.files[0];
  if (file) {
    let reader = new FileReader();
    reader.onload = function (e) {
      image1.src = e.target.result;
    };
    reader.readAsDataURL(file);
  }
});
document.getElementById("fileInput2").addEventListener("change", function (e) {
  let file = e.target.files[0];
  if (file) {
    let reader = new FileReader();
    reader.onload = function (e) {
      image2.src = e.target.result;
      original2Ctx.clearRect(
        0,
        0,
        original2Canvas.width,
        original2Canvas.height
      );
      original2Ctx.drawImage(image2, 0, 0, 300, 300);
    };
    reader.readAsDataURL(file);
  }
});

document.getElementById("mergeButton").addEventListener("click", function () {
  if (image1.complete && image2.complete) {
    mergingCanvas.width = 300;
    mergingCanvas.height = 300;
    mergingCtx.clearRect(0, 0, mergingCanvas.width, mergingCanvas.height);
    let alphaValue = parseFloat(document.getElementById("alphaInput").value);

    mergingCtx.globalAlpha = 1;
    mergingCtx.drawImage(originalCanvas, 0, 0, 300, 300);
    mergingCtx.globalAlpha = alphaValue;
    mergingCtx.drawImage(original2Canvas, 0, 0, 300, 300);

    mergingCtx.globalAlpha = 1;
    let mergingDataURL = mergingCanvas.toDataURL();
    mergingCanvas.style.display = "block";
    mergingCanvas.src = mergingDataURL;
  } else {
    alert("Please upload both images before merging.");
  }
});

document.getElementById("doMerging").addEventListener("click", () => {
  document.getElementsByClassName("mergingContainer")[0].style.display =
    "block";
  document.getElementsByClassName("menu")[0].style.display = "none";
});
document.getElementById("backButtonMerge").addEventListener("click", () => {
  document.getElementsByClassName("menu")[0].style.display = "block";
  document.getElementsByClassName("mergingContainer")[0].style.display = "none";
});
document.getElementsByClassName("mergingContainer")[0].style.display = "none";
