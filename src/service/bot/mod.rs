//! 机器人（Bot）服务
//!
//! 提供飞书机器人的完整功能集，支持机器人信息管理、消息处理、
//! 事件监听、API调用等智能机器人开发能力。是构建企业自动化助手的核心工具。
//!
//! # 核心功能
//!
//! ## 机器人信息管理
//! - 🤖 机器人基本信息查询
//! - ⚙️ 机器人配置和设置
//! - 📊 机器人状态监控
//! - 🔄 机器人权限管理
//! - 📈 机器人使用统计
//!
//! ## 消息处理
//! - 💬 消息接收和响应
//! - 📝 富文本消息处理
//! - 🖼️ 多媒体消息支持
//! - 🎯 消息路由分发
//! - 📊 消息统计分析
//!
//! ## 事件监听
//! - 🔔 实时事件接收
//! - 🔄 事件回调处理
//! - 📋 事件类型管理
//! - ⚡ 异步事件处理
//! - 📊 事件监控统计
//!
//! ## API调用
//! - 🚀 主动API调用能力
//! - 🔗 外部系统集成
//! - 📊 API调用统计
//! - 🔄 API请求重试机制
//! - 🛡️ API安全认证
//!
//! ## 智能交互
//! - 🧠 智能对话处理
//! - 🎯 意图识别理解
//! - 📋 上下文管理
//! - 🔄 多轮对话支持
//! - 📈 学习优化能力
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
//! // 获取机器人服务
//! let bot = &client.bot;
//!
//! // 获取机器人信息
//! // let info_request = GetBotInfoRequest::builder()
//! //     .bot_id("bot_123")
//! //     .build();
//! // let bot_info = bot.v3.info.get(info_request, None).await?;
//!
//! // 发送消息
//! // let message_request = SendMessageRequest::builder()
//! //     .receive_id("chat_456")
//! //     .msg_type("text")
//! //     .content(serde_json::json!({
//! //         "text": "你好，我是智能助手！"
//! //     }))
//! //     .build();
//! // bot.v3.message.send(message_request, None).await?;
//!
//! // 处理事件
//! // bot.v3.events.on_message_receive(|event| {
//! //     println!("收到消息: {:?}", event);
//! //     // 处理消息逻辑
//! // });
//!
//! // 调用API
//! // let api_request = CallAPIRequest::builder()
//! //     .api_path("/open-apis/contact/v3/users/me")
//! //     .method("GET")
//! //     .build();
//! // let api_response = bot.v3.api.call(api_request, None).await?;
//! ```
//!
//! # API版本
//!
//! 当前支持v3版本，提供最新的机器人功能：
//! - 机器人信息管理
//! - 高级消息处理
//! - 事件监听机制
//! - API调用能力
//! - 智能交互支持
//!
//! # 机器人特性
//!
//! - 🤖 智能对话处理
//! - 📱 多平台适配支持
//! - 🔔 实时事件响应
//! - 🔗 企业系统集成
//! - 📊 数据分析能力
//!
//! # 应用场景
//!
//! - 🏢 企业服务助手
//! - 📋 工作流程自动化
//! - 📊 数据查询和报告
//! - 🔔 通知和提醒服务
//! - 🎯 客户服务支持

pub mod models;
pub mod v3;

use crate::core::config::Config;

/// 机器人服务
pub struct BotService {
    /// v3版本API
    pub v3: v3::V3,
}

impl BotService {
    pub fn new(config: Config) -> Self {
        Self {
            v3: v3::V3::new(config),
        }
    }
}
