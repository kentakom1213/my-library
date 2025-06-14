<script lang="ts">
  import { onMount } from "svelte";

  const API_URL = "https://my-library.kentakom1213.workers.dev/books";

  type Book = {
    id: string;
    title: string;
    published_date: string;
    isbn: string;
    thumbnail_url: string;
    description: string;
    authors: string[];
  };

  // 書籍データを格納するリアクティブな変数
  let books: Book[] = [];
  // 読み込み状態とエラー情報を管理する変数
  let isLoading = true;
  let error: string | null = null;

  // コンポーネントがマウントされた後に実行されるライフサイクル関数
  onMount(async () => {
    try {
      // '/api/books' エンドポイントにリクエストを送信
      const response = await fetch(API_URL);

      if (!response.ok) {
        // HTTPステータスが 200-299 以外の場合はエラーを投げる
        throw new Error(`APIエラー: ${response.status}`);
      }

      // レスポンスボディをJSONとしてパース
      const data: Book[] = await response.json();
      books = data; // 取得したデータでbooksを更新
    } catch (e: any) {
      // ネットワークエラーやパースエラーなどをキャッチ
      error = e.message;
      console.error("データの取得に失敗しました:", e);
    } finally {
      // 成功・失敗にかかわらず、ローディング状態を解除
      isLoading = false;
    }
  });
</script>

<!-- 
  Tailwind CSSを使用する場合、ページの背景色などはグローバルに設定するのが一般的です。
  SvelteKitの場合、src/app.htmlの<body>タグに class="bg-gray-100" を追加するか、
  src/routes/+layout.svelte でグローバルなスタイルを適用してください。
-->
<main class="max-w-7xl mx-auto my-8 p-4 sm:p-8 bg-white rounded-xl shadow-lg">
  <h1 class="text-3xl font-bold text-center text-gray-800 mb-8">
    powell's library
  </h1>

  {#if isLoading}
    <p class="text-center text-lg text-gray-500 py-12">読み込み中...</p>
  {:else if error}
    <p class="text-center text-lg text-red-600 bg-red-50 p-8 rounded-lg">
      エラーが発生しました: {error}
    </p>
  {:else}
    <div
      class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-8"
    >
      {#each books as book (book.id)}
        <div
          class="flex flex-col bg-white border border-gray-200 rounded-lg shadow-md p-4 transition-all duration-200 ease-in-out hover:-translate-y-1 hover:shadow-xl"
        >
          <img
            src={book.thumbnail_url}
            alt="{book.title} の表紙"
            class="w-full h-auto mx-auto mb-4 rounded"
          />
          <h2 class="text-lg font-bold text-gray-900 leading-snug mb-2">
            {book.title}
          </h2>
          <p class="text-sm font-semibold text-gray-700">
            著者: {book.authors.join(", ")}
          </p>
          <p class="text-sm text-gray-600 leading-relaxed mt-3 flex-grow">
            {#if book.description}
              {book.description.length > 100
                ? book.description.substring(0, 100) + "..."
                : book.description}
            {:else}
              <span class="text-gray-400">説明はありません。</span>
            {/if}
          </p>
        </div>
      {/each}
    </div>
  {/if}
</main>
