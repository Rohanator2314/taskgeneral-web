import { createClient } from '@supabase/supabase-js';

const supabaseUrl = import.meta.env.VITE_SUPABASE_URL as string | undefined;
const supabaseAnonKey = import.meta.env.VITE_SUPABASE_ANON_KEY as string | undefined;

if (!supabaseUrl || !supabaseAnonKey) {
  throw new Error(
    'Missing Supabase env vars: VITE_SUPABASE_URL and VITE_SUPABASE_ANON_KEY must be set. ' +
    'Add them to frontend/.env locally or to your Vercel project environment variables.'
  );
}

export const supabase = createClient(supabaseUrl, supabaseAnonKey);
