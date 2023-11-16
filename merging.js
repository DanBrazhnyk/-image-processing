let originalCanvas = document.getElementById("originalCanvas");
let originalCtx = originalCanvas.getContext("2d");

let original2Canvas = document.getElementById("original2Canvas");
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
  document.getElementById("alphaLabel").style.display = "block";
  document.getElementById("alphaInput").style.display = "flex";
  document.getElementById("doBlur").style.display = "none";
  document.getElementById("doMerging").style.display = "none";
  document.getElementById("mergeButton").style.display = "flex";
  document.getElementById("blurButton").style.display = "none";
  document.getElementById("backButton").style.display = "flex";
  document.getElementById("blurLevel").style.display = "none";
  document.getElementById("blurredCanvas").style.display = "none";
  document.getElementById("doBlur").style.display = "none";  
  document.getElementById("choose").style.display = "none";
  document.getElementById("imgInputes").style.display = "flex";
  document.getElementById("fileInput2").style.display = "flex";
  document.getElementById("original2Canvas").style.display = "flex";
});
