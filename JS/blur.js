const boxBlurImage = (imageData, radius) => {
  const startTime = performance.now();
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
  const endTime = performance.now();
  const elapsedTimeInSeconds = (endTime - startTime) / 1000; 
  console.log(`Time taken: ${elapsedTimeInSeconds.toFixed(3)} seconds`);
  return new ImageData(blurredData, width, height);
};

document.getElementById("blurInput").addEventListener("change", (e) => {
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
  document.getElementsByClassName("blurContainer")[0].style.display = "block";
  document.getElementsByClassName("menu")[0].style.display = "none";
});
document.getElementsByClassName("blurContainer")[0].style.display = "none";
document.getElementById("backButton").addEventListener("click", () => {
  document.getElementsByClassName("blurContainer")[0].style.display = "none";
  document.getElementsByClassName("menu")[0].style.display = "block";
});
