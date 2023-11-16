const boxBlurImage = (imageData, radius) => {
  const data = imageData.data;
  const width = imageData.width;
  const height = imageData.height;
  const blurredData = new Uint8ClampedArray(data.length);

  const boxWidth = Math.floor(radius / 2);
  const boxHeight = Math.floor(radius / 2);

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      let red = 0,
        green = 0,
        blue = 0,
        count = 0;

      for (let dy = -boxHeight; dy <= boxHeight; dy++) {
        for (let dx = -boxWidth; dx <= boxWidth; dx++) {
          const nx = Math.min(Math.max(x + dx, 0), width - 1);
          const ny = Math.min(Math.max(y + dy, 0), height - 1);

          const offset = (ny * width + nx) * 4;
          red += data[offset];
          green += data[offset + 1];
          blue += data[offset + 2];
          count++;
        }
      }

      const offset = (y * width + x) * 4;
      blurredData[offset] = red / count;
      blurredData[offset + 1] = green / count;
      blurredData[offset + 2] = blue / count;
      blurredData[offset + 3] = data[offset + 3];
    }
  }

  return new ImageData(blurredData, width, height);
};

document.getElementById("fileInput").addEventListener("change", (e) => {
  const originalCanvas = document.getElementById("originalCanvas");
  const originalCtx = originalCanvas.getContext("2d");
  const blurredCanvas = document.getElementById("blurredCanvas");
  const blurredCtx = blurredCanvas.getContext("2d");
  const img = new Image();

  img.onload = () => {
    originalCanvas.width = img.width;
    originalCanvas.height = img.height;
    blurredCanvas.width = img.width;
    blurredCanvas.height = img.height;

    originalCtx.drawImage(img, 0, 0);
  };

  img.src = URL.createObjectURL(e.target.files[0]);
});

document.getElementById("blurButton").addEventListener("click", () => {
  const originalCanvas = document.getElementById("originalCanvas");
  const originalCtx = originalCanvas.getContext("2d");
  const blurredCanvas = document.getElementById("blurredCanvas");
  const blurredCtx = blurredCanvas.getContext("2d");

  const imageData = originalCtx.getImageData(
    0,
    0,
    originalCanvas.width,
    originalCanvas.height
  );

  const radius = document.getElementById("blurLevel").value;

  const blurredData = boxBlurImage(imageData, radius);
  blurredCtx.putImageData(blurredData, 0, 0);
});
document.getElementById("doBlur").addEventListener("click", () => {
  document.getElementById("doMerging").style.display = "none";
  document.getElementById("mergeButton").style.display = "none";
  document.getElementById("blurButton").style.display = "flex";
  document.getElementById("backButton").style.display = "flex";
  document.getElementById("blurLevel").style.display = "flex";
  document.getElementById("blurredCanvas").style.display = "flex";
  document.getElementById("doBlur").style.display = "none";
  document.getElementById("imgInputes").style.display = "flex";
  document.getElementById("fileInput2").style.display = "none";
    document.getElementById("choose").style.display = "none";
});
document.getElementById("mergingCanvas").style.display = "none";
document.getElementById("blurButton").style.display = "none";
document.getElementById("mergeButton").style.display = "none";
document.getElementById("backButton").style.display = "none";
document.getElementById("blurLevel").style.display = "none";
document.getElementById("blurredCanvas").style.display = "none";
document.getElementById("imgInputes").style.display = "none";
document.getElementById("alphaInput").style.display = "none";
document.getElementById("alphaLabel").style.display = "none";

document.getElementById("backButton").addEventListener("click", () => {
  document.getElementById("alphaInput").style.display = "none";
  document.getElementById("doMerging").style.display = "flex";
  document.getElementById("mergingCanvas").style.display = "none";
  document.getElementById("blurButton").style.display = "none";
  document.getElementById("mergeButton").style.display = "none";
  document.getElementById("blurLevel").style.display = "none";
  document.getElementById("doBlur").style.display = "flex";
  document.getElementById("choose").style.display = "flex";
  document.getElementById("backButton").style.display = "none";
  document.getElementById("imgInputes").style.display = "none";
  document.getElementById("alphaLabel").style.display = "none";
  document.getElementById("original2Canvas").style.display = "none";
  const originalCanvas = document.getElementById("originalCanvas");
  const originalCtx = originalCanvas.getContext("2d");
  originalCtx.clearRect(0, 0, originalCanvas.width, originalCanvas.height);  

  const original2Canvas = document.getElementById("original2Canvas");
  const original2Ctx = original2Canvas.getContext("2d");
  original2Ctx.clearRect(0, 0, original2Canvas.width, originalCanvas.height);

  const blurredCanvas = document.getElementById("blurredCanvas");
  const blurredCtx = blurredCanvas.getContext("2d");
  blurredCtx.clearRect(0, 0, blurredCanvas.width, blurredCanvas.height);
});
