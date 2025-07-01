//! 飞书智能伙伴创建平台（Aily）服务
//!
//! 提供飞书智能伙伴创建平台的完整功能集，支持会话管理、消息处理、
//! 运行控制、技能调用、知识问答等AI智能助手开发能力。是构建企业AI应用的核心平台。
//!
//! # 核心功能
//!
//! ## 会话管理
//! - 💬 智能会话创建和管理
//! - ⚙️ 会话配置和个性化
//! - 📊 会话状态跟踪监控
//! - 🔄 多会话并发处理
//! - 📈 会话质量分析评估
//!
//! ## 消息处理
//! - 📝 消息发送和接收处理
//! - 📋 消息历史记录管理
//! - 🎯 消息路由和分发
//! - 📊 消息统计分析
//! - 🔄 消息同步和备份
//!
//! ## 运行控制
//! - 🚀 AI运行任务创建执行
//! - 📊 运行状态实时监控
//! - ⏸️ 运行过程控制管理
//! - 📈 运行性能分析优化
//! - 🔄 运行结果处理反馈
//!
//! ## 技能调用
//! - 🛠️ 智能技能调用执行
//! - 📋 技能信息查询管理
//! - 📊 技能性能统计分析
//! - 🔗 技能组合编排
//! - ⚡ 技能快速响应机制
//!
//! ## 知识问答
//! - 🧠 智能知识库问答
//! - 📚 知识库管理维护
//! - 📁 文件上传处理分析
//! - 🔍 知识检索和匹配
//! - 📈 问答质量优化
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuilt)
//!     .build();
//!
//! // 获取智能伙伴服务
//! let aily = &client.aily;
//!
//! // 创建会话
//! // let session_request = CreateSessionRequest::builder()
//! //     .assistant_id("assistant_123")
//! //     .user_id("user_456")
//! //     .build();
//! // let session = aily.session.create(session_request, None).await?;
//!
//! // 发送消息
//! // let message_request = CreateMessageRequest::builder()
//! //     .session_id("session_789")
//! //     .content("请帮我分析最新的销售数据")
//! //     .message_type("text")
//! //     .build();
//! // let message = aily.message.create(message_request, None).await?;
//!
//! // 创建运行
//! // let run_request = CreateRunRequest::builder()
//! //     .session_id("session_789")
//! //     .assistant_id("assistant_123")
//! //     .build();
//! // let run = aily.run.create(run_request, None).await?;
//!
//! // 调用技能
//! // let skill_request = CallSkillRequest::builder()
//! //     .skill_id("skill_456")
//! //     .parameters(serde_json::json!({
//! //         "data_source": "sales_db",
//! //         "time_range": "last_month"
//! //     }))
//! //     .build();
//! // let skill_result = aily.skill.call(skill_request, None).await?;
//!
//! // 知识问答
//! // let qa_request = KnowledgeQARequest::builder()
//! //     .question("什么是公司的销售策略？")
//! //     .knowledge_base_id("kb_789")
//! //     .build();
//! // let answer = aily.knowledge.qa(qa_request, None).await?;
//! ```
//!
//! # AI平台特性
//!
//! - 🤖 先进的AI智能处理
//! - 🧠 深度学习模型支持
//! - 📊 数据驱动的智能决策
//! - 🔗 企业系统无缝集成
//! - ⚡ 高性能实时响应
//!
//! # 智能应用
//!
//! - 💼 企业智能助手
//! - 📊 数据分析和洞察
//! - 🎯 个性化推荐服务
//! - 🔄 工作流程自动化
//! - 📈 业务决策支持

use crate::core::config::Config;

pub mod knowledge;
pub mod message;
pub mod models;
pub mod run;
pub mod session;
pub mod skill;

use knowledge::KnowledgeService;
use message::MessageService;
use run::RunService;
use session::SessionService;
use skill::SkillService;

/// 飞书智能伙伴创建平台服务
///
/// 提供飞书智能伙伴创建平台（aily）的完整功能，包括：
/// - 会话管理：创建、更新、查询、删除智能伙伴会话
/// - 消息管理：发送消息、获取消息、列出消息历史
/// - 运行管理：创建运行、查询运行状态、取消运行
/// - 技能管理：调用技能、获取技能信息、查询技能列表
/// - 知识问答：数据知识问答、知识库管理、文件上传处理
pub struct AilyService {
    /// 会话管理服务
    pub session: SessionService,
    /// 消息管理服务
    pub message: MessageService,
    /// 运行管理服务
    pub run: RunService,
    /// 技能管理服务
    pub skill: SkillService,
    /// 知识问答服务
    pub knowledge: KnowledgeService,
}

impl AilyService {
    pub fn new(config: Config) -> Self {
        Self {
            session: SessionService::new(config.clone()),
            message: MessageService::new(config.clone()),
            run: RunService::new(config.clone()),
            skill: SkillService::new(config.clone()),
            knowledge: KnowledgeService::new(config),
        }
    }
}
