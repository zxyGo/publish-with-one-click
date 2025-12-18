import Database from '@tauri-apps/plugin-sql';

/**
 * 数据库单例管理类
 */
class DatabaseService {
  private static instance: DatabaseService;
  private db: Database | null = null;
  // 数据库文件名为 myapp.db，位于应用数据目录
  private readonly DB_NAME = 'sqlite:myapp.db';

  private constructor() {}

  /**
   * 获取 DatabaseService 单例
   */
  public static getInstance(): DatabaseService {
    if (!DatabaseService.instance) {
      DatabaseService.instance = new DatabaseService();
    }
    return DatabaseService.instance;
  }

  /**
   * 初始化数据库连接
   */
  public async init(): Promise<void> {
    if (this.db) return;
    try {
      // 连接数据库，如果不存在则会自动创建
      this.db = await Database.load(this.DB_NAME);
      console.log('Database connected successfully');
      await this.initTables();
    } catch (error) {
      console.error('Failed to connect to database:', error);
      throw error;
    }
  }

  /**
   * 初始化数据表
   * 在这里定义你的表结构
   */
  private async initTables(): Promise<void> {
    if (!this.db) return;
    
    // 示例：创建一个 users 表
    // 实际项目中建议使用迁移工具或手动管理版本
    const query = `
      CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )
    `;
    await this.db.execute(query);
  }

  /**
   * 获取数据库实例
   */
  public getDb(): Database {
    if (!this.db) {
      throw new Error('Database not initialized. Call init() first.');
    }
    return this.db;
  }
}

export const dbService = DatabaseService.getInstance();
