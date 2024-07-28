import Link from "next/link";

export default function Home() {
  return (
    <div>
      <h1>Welcome to the Main Page</h1>
      <p>This is the main page of our Next.js application.</p>
      <Link href="/healthcheck">
        <p>Go to API Access Page</p>
      </Link>
    </div>
  );
}
