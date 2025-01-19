document.addEventListener("DOMContentLoaded", () => {
  function initFlatpickr() {
    flatpickr("#datepicker", {
      dateFormat: "Y/m/d", // 日付フォーマット（例: 2025/01/17）
      locale: "ja", // 日本語対応
    });

    // カレンダーアイコンがクリックされたときにカレンダーを表示
    document
      .getElementById("calendar-icon")
      .addEventListener("click", function () {
        const datepicker = document.getElementById("datepicker");
        flatpickr(datepicker).open(); // アイコンクリックでカレンダーを開く
      });
  }

  function attachButtonListeners() {
    const buttons = document.querySelectorAll("button");
    buttons.forEach((button) => {
      button.addEventListener("click", (event) => {
        const form = button.closest("form"); // ボタンに対応するフォームを取得
        const taskId = form.querySelector("#id").value;

        if (button.id.startsWith("start")) {
          sendPostRequest("/start", taskId);
        } else if (button.id.startsWith("delete")) {
          sendPostRequest("/delete", taskId);
        } else if (button.id.startsWith("done")) {
          sendPostRequest("/done", taskId);
        } else if (button.id.startsWith("undo")) {
          sendPostRequest("/undo", taskId);
        } else if (button.id.startsWith("doing")) {
          sendPostRequest("/doing", taskId);
        }
      });
    });
  }

  function sendPostRequest(url, taskId) {
    const params = new URLSearchParams();
    params.append("id", taskId);

    fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/x-www-form-urlencoded",
      },
      body: params.toString(),
    })
      .then((response) => {
        if (response.ok) {
          return response.text(); // サーバーから新しいHTMLを取得
        } else {
          throw new Error("Network response was not ok");
        }
      })
      .then((updatedHTML) => {
        console.log("Success");

        // HTML全体を置き換え
        document.documentElement.innerHTML = updatedHTML;

        // 新しいHTMLに対してイベントリスナーを再設定
        attachButtonListeners();
        initFlatpickr();
      })
      .catch((error) => {
        console.error("There was a problem with the fetch operation:", error);
      });
  }

  // 初期読み込み時にイベントを設定
  attachButtonListeners();
  initFlatpickr();
});
