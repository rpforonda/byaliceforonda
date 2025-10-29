<script>
  let answers = $state({});
  let result = $state(null);

  const siteUrl = $derived(
    typeof window !== 'undefined' ? window.location.origin : 'https://example.com'
  );

  const questions = [
    { id: 'q1', text: 'Pick a trait', options: ['Brave', 'Clever', 'Kind'] },
    { id: 'q2', text: 'Choose a setting', options: ['Forest', 'City', 'Sea'] },
    { id: 'q3', text: 'Pick a companion', options: ['Wolf', 'Owl', 'Fox'] }
  ];

  const computeResult = () => {
    const score = { Brave: 0, Clever: 0, Kind: 0 };
    for (const q of questions) {
      const v = answers[q.id];
      if (v && score[v] != null) score[v]++;
    }
    const top = Object.entries(score).sort((a, b) => b[1] - a[1])[0];
    return top ? top[0] : 'Adventurer';
  };

  function submit() {
    result = computeResult();
    if (typeof localStorage !== 'undefined') localStorage.setItem('quizResult', result);
  }

  const openWin = (url) => {
    const w = 600, h = 550;
    const y = window.top.outerHeight / 2 + window.top.screenY - (h / 2);
    const x = window.top.outerWidth / 2 + window.top.screenX - (w / 2);
    window.open(url, 'share', `width=${w},height=${h},left=${x},top=${y}`);
  };

  const shareTwitter = () => {
    if (!result) return;
    const text = encodeURIComponent(`I got \"${result}\" in the Character Quiz!`);
    const url = encodeURIComponent(siteUrl + '/quiz');
    openWin(`https://twitter.com/intent/tweet?text=${text}&url=${url}`);
  };

  const shareFacebook = () => {
    if (!result) return;
    const url = encodeURIComponent(siteUrl + '/quiz');
    openWin(`https://www.facebook.com/sharer/sharer.php?u=${url}`);
  };

  const shareLinkedIn = () => {
    if (!result) return;
    const url = encodeURIComponent(siteUrl + '/quiz');
    openWin(`https://www.linkedin.com/sharing/share-offsite/?url=${url}`);
  };

  const shareNative = async () => {
    if (!result || typeof navigator === 'undefined' || !navigator.share) return;
    try {
      await navigator.share({
        title: 'Character Quiz Result',
        text: `I got \"${result}\"! Which character are you?`,
        url: siteUrl + '/quiz'
      });
    } catch {}
  };

  const copySnippet = async () => {
    if (!result || typeof navigator === 'undefined' || !navigator.clipboard) return;
    await navigator.clipboard.writeText(`I got \"${result}\" in the Character Quiz! ${siteUrl}/quiz`);
    alert('Copied to clipboard!');
  };
</script>

<div class="grid gap-4">
  {#each questions as q}
    <div class="py-3 border-b border-gray-100">
      <div class="font-semibold mb-2">{q.text}</div>
      <div class="flex flex-wrap gap-3">
        {#each q.options as opt}
          <label class="inline-flex items-center gap-2">
            <input type="radio" class="accent-black" name={q.id} value={opt} onchange={(e) => (answers[q.id] = e.currentTarget.value)} />
            <span>{opt}</span>
          </label>
        {/each}
      </div>
    </div>
  {/each}

  <button class="px-4 py-2 rounded-lg border border-gray-200 bg-white hover:bg-gray-50 w-fit" onclick={submit}>
    See your character
  </button>

  {#if result}
    <div class="mt-2 p-3 bg-gray-50 border border-gray-100 rounded-lg">
      <h3 class="font-semibold">You’re most like: <span class="font-bold">{result}</span></h3>
      <div class="flex flex-wrap gap-2 mt-3">
        <button class="px-3 py-2 rounded-lg border border-gray-200 bg-white hover:bg-gray-50" onclick={shareTwitter}>Share on X</button>
        <button class="px-3 py-2 rounded-lg border border-gray-200 bg-white hover:bg-gray-50" onclick={shareFacebook}>Facebook</button>
        <button class="px-3 py-2 rounded-lg border border-gray-200 bg-white hover:bg-gray-50" onclick={shareLinkedIn}>LinkedIn</button>
        {#if typeof navigator !== 'undefined' && navigator.share}
          <button class="px-3 py-2 rounded-lg border border-gray-200 bg-white hover:bg-gray-50" onclick={shareNative}>Share…</button>
        {/if}
        <button class="px-3 py-2 rounded-lg border border-gray-200 bg-white hover:bg-gray-50" onclick={copySnippet}>Copy text</button>
      </div>
    </div>
  {/if}
</div>
