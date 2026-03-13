export const BG_MUSIC_KEY = Symbol("bgMusic");

export type BgMusicContext = {
  playing: boolean;
  toggle: () => void;
};
