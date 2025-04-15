window.onload = async () => {
  const result = await window.__TAURI__.invoke("list_zip_images_smb", {
    smbUrl: "smb://192.168.1.10/shared/manga.cbz",
    username: "guest",
    password: ""
  });

  console.log("Images in ZIP:", result);
};
