import { useEffect } from "react";
import { useRouter } from "next/router";
import Error from "next/error";

export default function Custom404() {
  const router = useRouter();
  useEffect(() => {
    setTimeout(() => router.push("/"), 400);
  });
  return <Error statusCode={404} title={"Page not found"} />;
}
