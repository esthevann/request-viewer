import Database from "tauri-plugin-sql-api";
import { Request } from "../models/Request";

let db: Database | null = null;
const load = Database.load('sqlite:test.db').then(instance => {
    db = instance
    return db
})

export async function all(): Promise<Request[]> {
    await load
    if (db) {
        return await db.select('SELECT * FROM requests')
    }
    throw new Error("Couldn't fetch requests")
}

export async function create(address: string): Promise<Request>{
    await load;
    if (db){
        const {lastInsertId: id} = await db?.execute('INSERT INTO requests (address) VALUES ($1)', [address])
        return {
            id,
            address
        }
    }
    throw new Error("Couldn't create request")
}
