import { useRef, useState } from "react";
import { Button } from "react-bootstrap";
import { FaBookmark, FaRegBookmark } from "react-icons/fa";

import type { RegisterVolunteerFavoriteRequestBody } from "@/__generated__/command";

import { apiClientVolunteer } from "@/api/command";
import { useAuthContext } from "@/contexts/AuthContext";

type Props = {
  vid: string;
  initIsFav: boolean;
  onFavClick?: () => void;
  onUnFavClick?: () => void;
};

const noop = () => {};

export const FavButton = ({
  vid,
  initIsFav,
  onFavClick = noop,
  onUnFavClick = noop,
}: Props) => {
  const [isFav, setIsFav] = useState(initIsFav);

  const isFavButtonDisabled = useRef(false);

  const { user } = useAuthContext();

  if (!user) {
    return null;
  }

  const fav = async () => {
    if (isFavButtonDisabled.current) return;
    isFavButtonDisabled.current = true;

    try {
      const body: RegisterVolunteerFavoriteRequestBody = {
        uid: user.uid,
        vid,
      };
      await apiClientVolunteer.registerFavorite(body);
      onFavClick();
      setIsFav(true);
    } catch (e) {
      alert("お気に入り登録に失敗しました");
    }

    isFavButtonDisabled.current = false;
  };

  const unFav = async () => {
    if (isFavButtonDisabled.current) return;
    isFavButtonDisabled.current = true;

    try {
      const body: RegisterVolunteerFavoriteRequestBody = {
        uid: user.uid,
        vid,
      };
      await apiClientVolunteer.unregisterFavorite(body);
      onUnFavClick();
      setIsFav(false);
    } catch (e) {
      alert("お気に入り解除に失敗しました");
    }

    isFavButtonDisabled.current = false;
  };

  return isFav ? (
    <Button variant="none" onClick={unFav}>
      <FaBookmark />
    </Button>
  ) : (
    <Button variant="none" onClick={fav}>
      <FaRegBookmark />
    </Button>
  );
};
