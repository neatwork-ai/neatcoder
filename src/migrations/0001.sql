-- PostgreSQL migrations

CREATE TYPE data_flow as ENUM ('Input', 'Output');
CREATE TYPE initiator as ENUM ('Human', 'Output');

-- stores chat history
CREATE TABLE messages (
    id uuid PRIMARY KEY,
    -- the role is e.g "assistant" or "user"
    role TEXT NOT NULL,
    -- the prompt text
    content TEXT NOT NULL,
    -- approximation for how many tokens the content has
    tokens INTEGER NOT NULL,
    -- indicates if a given message is an input to or an output of a prompt
    flow data_flow,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS inputs (
    id uuid PRIMARY KEY,
    -- Array representing the chain of inputs and outputs that led to this state
    causal_chain uuid[],
    -- The parser object who is responsible for serializing and the deserializing
    -- the text into and from an object
    parser TEXT,
);

CREATE TABLE IF NOT EXISTS outputs (
    id uuid PRIMARY KEY,
    -- Array representing the chain of inputs and outputs that led to this state
    causal_chain uuid[],
    -- Array represeting the messages chainned in and handed-over to the LLM as context history
    context_chain uuid[],
    -- What was the LLM model that produced this output
    model TEXT NOT NULL,
    -- The parser object who is responsible for serializing and the deserializing
    -- the text into and from an object
    parser TEXT,
);