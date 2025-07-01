import { MONGO_URL } from '$env/static/private';
import { type SessionResult } from '$lib/models/session_result';
import { Collection, Db, MongoClient } from 'mongodb';

const client = new MongoClient(MONGO_URL);
export let db: Db;
let resultsCollection: Collection<SessionResult>;

export async function connect(): Promise<void> {
    console.log("Connecting to ", MONGO_URL);
    await client.connect();
    db = client.db();
    resultsCollection = db.collection<SessionResult>('session_results');
    console.log("Using collection:", resultsCollection.collectionName);
}

export async function saveResult(result: SessionResult): Promise<void> {
    assertCollectionExists()

    await resultsCollection.insertOne(result);

}

export async function getResults(): Promise<SessionResult[]> {
    assertCollectionExists()

    return resultsCollection
        .find({})
        .sort({ _id: -1 })
        .toArray();
}

export async function updateName(id: string, name: string): Promise<SessionResult> {
    assertCollectionExists()

    const result = await resultsCollection
        .findOneAndUpdate(
            { id },
            { $set: { name: name } },
            { returnDocument: 'after' }
        )

    if (!result) {
        throw new Error(`Failed to update SessionResult with id ${id}`)
    }

    return {
        id: result.id,
        name: result.name,
        rate: result.rate,
        duration: result.duration,
    };
}

function assertCollectionExists() {
    if (!resultsCollection) {
        throw new Error("Not connected to MongoDB");
    }
}
