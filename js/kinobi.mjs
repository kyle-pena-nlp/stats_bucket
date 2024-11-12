import * as path from 'path';
import { fileURLToPath } from 'url';
import { dirname } from 'path';
import { createFromIdls, renderJavaScriptVisitor } from "@metaplex-foundation/kinobi";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Instantiate Kinobi.
const kinobi = createFromIdls([
  path.join(__dirname, "idl", "stats_bucket.json"),
]);

// Update the Kinobi tree using visitors...

// Render JavaScript.
const jsDir = path.join(__dirname, "src", "generated");
kinobi.accept(renderJavaScriptVisitor(jsDir));