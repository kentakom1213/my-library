<script lang="ts">
  import { env } from "$env/dynamic/public";

  // APIからのレスポンスの型を定義
  type ApiResponse = {
    message?: string;
    error?: string;
    isbn?: string | null;
  };

  let isbn = "";
  let token = "";
  let message = "";
  let isLoading = false;

  // .envファイルからバックエンドのURLを取得。なければローカルのデフォルト値を使う
  const backendUrl = env.PUBLIC_BACKEND_URL || "http://127.0.0.1:8787";

  async function handleSubmit() {
    if (!isbn || !token) {
      message = "ISBNとトークンを両方入力してください。";
      return;
    }

    isLoading = true;
    message = "リクエストを送信中...";

    try {
      const response = await fetch(`${backendUrl}/book/${isbn}`, {
        method: "POST",
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      if (response.headers.get("Content-Type")?.includes("application/json")) {
        // response.json() の結果を ApiResponse 型として扱う
        const data: ApiResponse = await response.json();

        if (response.ok) {
          message = `✅ 成功: ${data.message || "書籍を追加しました。"}`;
        } else {
          message = `❌ エラー (${response.status}): ${
            data.message || data.error || "不明なエラーが発生しました。"
          }`;
        }
      } else {
        const text = await response.text();
        message = `❌ エラー (${response.status}): ${text}`;
      }
    } catch (error) {
      console.error("Fetch error:", error);
      message =
        "リクエスト中に予期せぬエラーが発生しました。コンソールを確認してください。";
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="container mx-auto max-w-lg p-8">
  <div class="flex justify-between items-center mb-6">
    <h1 class="text-2xl font-bold">管理者ページ: 書籍の追加</h1>
    <a
      href="/"
      class="text-sm font-medium text-indigo-600 hover:text-indigo-800 transition-colors"
    >
      ← ホームへ戻る
    </a>
  </div>

  <form
    on:submit|preventDefault={handleSubmit}
    class="space-y-4 bg-white p-6 rounded-lg shadow-md"
  >
    <div>
      <label for="isbn" class="block text-sm font-medium text-gray-700"
        >ISBN</label
      >
      <input
        type="text"
        id="isbn"
        bind:value={isbn}
        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
        placeholder="例: 9784873119593"
        required
      />
    </div>

    <div>
      <label for="token" class="block text-sm font-medium text-gray-700"
        >認証トークン</label
      >
      <input
        type="password"
        id="token"
        bind:value={token}
        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
        placeholder="APIトークンを入力"
        required
      />
    </div>

    <button
      type="submit"
      disabled={isLoading}
      class="inline-flex w-full justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 disabled:bg-gray-400 disabled:cursor-not-allowed"
    >
      {#if isLoading}
        送信中...
      {:else}
        書籍を追加
      {/if}
    </button>
  </form>

  {#if message}
    <div
      class="mt-6 rounded-md p-4"
      class:bg-green-100={message.startsWith("✅")}
      class:text-green-800={message.startsWith("✅")}
      class:bg-red-100={message.startsWith("❌")}
      class:text-red-800={message.startsWith("❌")}
      class:bg-blue-100={!message.startsWith("✅") && !message.startsWith("❌")}
      class:text-blue-800={!message.startsWith("✅") &&
        !message.startsWith("❌")}
    >
      <p>{message}</p>
    </div>
  {/if}
</div>
