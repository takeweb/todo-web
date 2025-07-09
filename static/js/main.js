document.addEventListener("DOMContentLoaded", () => {
  const baseUrlMeta = document.querySelector('meta[name="base-url"]');
  const APP_BASE_URL = baseUrlMeta ? baseUrlMeta.content : ""; // メタタグがなければ空文字列

  // Sortableインスタンスをグローバルスコープに置いて、必要に応じて破棄できるようにします（厳密にはこのアプローチでは不要ですが）
  let unstartedSortable;
  let inProgressSortable;
  let completedSortable;

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
      // 重複を防ぐため、既存のリスナーを削除してから追加
      button.removeEventListener("click", handleButtonClick);
      button.addEventListener("click", handleButtonClick);
    });
  }

  function handleButtonClick(event) {
    const button = event.currentTarget;
    const form = button.closest("form"); // ボタンに対応するフォームを取得
    const taskId = form.querySelector("input[name='id']").value;

    if (button.id.startsWith("start")) {
      sendPostRequest(`${APP_BASE_URL}/start`, taskId);
    } else if (button.id.startsWith("delete")) {
      sendPostRequest(`${APP_BASE_URL}/delete`, taskId);
    } else if (button.id.startsWith("done")) {
      sendPostRequest(`${APP_BASE_URL}/done`, taskId);
    } else if (button.id.startsWith("undo")) {
      sendPostRequest(`${APP_BASE_URL}/undo`, taskId);
    } else if (button.id.startsWith("doing")) {
      sendPostRequest(`${APP_BASE_URL}/doing`, taskId);
    }
  }

  function attachDragAndDrop() {
    const unstartedList = document.getElementById("unstarted-tasks-list");
    const inProgressList = document.getElementById("in-progress-tasks-list");
    const completedList = document.getElementById("completed-tasks-list");

    // 既存のSortableインスタンスがあれば破棄する
    if (unstartedSortable) {
      unstartedSortable.destroy();
    }
    if (inProgressSortable) {
      inProgressSortable.destroy();
    }
    if (completedSortable) {
      completedSortable.destroy();
    }

    const sortableOptions = {
      group: "tasks",
      draggable: ".col-12[data-task-id]",
      animation: 150,
      ghostClass: "sortable-ghost",
      onEnd: function (evt) {
        const item = evt.item;
        const toList = evt.to;

        const taskId = item.dataset.taskId;
        let url = "";

        switch (toList.id) {
          case "unstarted-tasks-list":
            url = `${APP_BASE_URL}/undo`;
            break;
          case "in-progress-tasks-list":
            url = `${APP_BASE_URL}/start`;
            break;
          case "completed-tasks-list":
            url = `${APP_BASE_URL}/done`;
            break;
        }

        if (taskId && url) {
          sendPostRequest(url, taskId);
        }
      },
    };

    // 新しいSortableインスタンスを初期化して変数に代入
    unstartedSortable = new Sortable(unstartedList, sortableOptions);
    inProgressSortable = new Sortable(inProgressList, sortableOptions);
    completedSortable = new Sortable(completedList, sortableOptions);
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
          return response.text();
        } else {
          throw new Error("Network response was not ok");
        }
      })
      .then((updatedHTML) => {
        console.log("Success");

        // HTML全体を置き換え
        document.documentElement.innerHTML = updatedHTML;

        // HTMLが置き換わった後、すべてのイベントリスナーとSortableインスタンスを再アタッチ/再初期化
        initFlatpickr(); // flatpickrも再初期化
        attachButtonListeners();
        attachDragAndDrop(); // Sortable.jsのインスタンスを破棄してから再初期化
      })
      .catch((error) => {
        console.error("There was a problem with the fetch operation:", error);
      });
  }

  // DOMが完全にロードされたときの初期設定
  attachButtonListeners();
  initFlatpickr();
  attachDragAndDrop();
});
