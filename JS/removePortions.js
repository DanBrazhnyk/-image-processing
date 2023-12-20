document.addEventListener("DOMContentLoaded", () => {
  const imageInput = document.getElementById("imageInput");
  const originalImage = document.getElementById("originalImage");
  const removedCanvas = document.getElementById("removedCanvas");
  const removeXInput = document.getElementById("removeX");
  const removeYInput = document.getElementById("removeY");
  const removeWidthInput = document.getElementById("removeWidth");
  const removeHeightInput = document.getElementById("removeHeight");
  const removeRadiusInput = document.getElementById("removeRadius");
  const removeButton = document.getElementById("removePortionButton");
  const doRemovingButton = document.getElementById("doRemovingButton");
  const removeContainer = document.getElementById("removeContainer");
  let layers = [];

  imageInput.addEventListener("change", handleFileSelect);
  removeButton.addEventListener("click", removePortion);
  doRemovingButton.addEventListener("click", () => {
      removeContainer.style.display = "block";
      document.getElementById("blurContainer").style.display = "none";
      document.getElementById("mergeContainer").style.display = "none";
  });

  function handleFileSelect(event) {
      const file = event.target.files[0];

      if (file) {
          const reader = new FileReader();

          reader.onload = function (e) {
              originalImage.src = e.target.result;
              originalImage.onload = function () {
                  createLayer(originalImage);
              };
          };

          reader.readAsDataURL(file);
      }
  }

  function createLayer(image) {
      const canvas = document.createElement("canvas");
      canvas.width = image.naturalWidth;
      canvas.height = image.naturalHeight;
      const ctx = canvas.getContext("2d");
      ctx.drawImage(image, 0, 0, canvas.width, canvas.height);
      layers.push(canvas);
      viewImage();
  }

  function viewImage() {
      removedCanvas.width = originalImage.naturalWidth;
      removedCanvas.height = originalImage.naturalHeight;
      const ctx = removedCanvas.getContext("2d");
      ctx.clearRect(0, 0, removedCanvas.width, removedCanvas.height);

      layers.forEach((layer) => {
          ctx.drawImage(layer, 0, 0, removedCanvas.width, removedCanvas.height);
      });
  }

  function removePortion() {
      const x = parseInt(removeXInput.value);
      const y = parseInt(removeYInput.value);
      const width = parseInt(removeWidthInput.value);
      const height = parseInt(removeHeightInput.value);
      const radius = parseInt(removeRadiusInput.value);

      const layer = layers[layers.length - 1];
      const ctx = layer.getContext("2d");

      if (radius > 0) {
          ctx.beginPath();
          ctx.arc(x + radius, y + radius, radius, 0, 2 * Math.PI);
          ctx.fillStyle = "white";
          ctx.fill();
          ctx.closePath();
      } else {
          ctx.fillStyle = "white";
          ctx.fillRect(x, y, width, height);
      }

      const newImage = new Image();
      newImage.onload = function () {
          createLayer(newImage);
      };
      newImage.src = layer.toDataURL();
  }
});

document.getElementsByClassName("removeContainer")[0].style.display = "none";
document.getElementById("doRemoving").addEventListener("click", () => {
  document.getElementsByClassName("removeContainer")[0].style.display = "block";
  document.getElementsByClassName("menu")[0].style.display = "none";
});

document.getElementById("backButtonRemove").addEventListener("click", () => {
  document.getElementsByClassName("menu")[0].style.display = "block";
  document.getElementsByClassName("removeContainer")[0].style.display = "none";
});
