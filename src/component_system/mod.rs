//! Un composant est une partie du bot. 
//! 
//! Un composant à un domaine défini, tel que là modération, la gestion des tickets ou d'autres taches du genre.
//! Chacun peuvent contenir un set de commandes et d'acquisition événements et réponde à certaines taches de leur domaine.
//! Un composant est sensé s'autogérer mais Mais rien n'empêche la communication entre ces derniers.

use std::sync::Arc;
use serenity::async_trait;

mod event;
mod framework;
mod data;
mod slash;
pub mod command_parser;
pub mod components;
pub mod manager;

pub use event::EventDispatcher;
pub use framework::{Framework , FrameworkConfig};
pub use framework::{Context, Message};
pub use serenity::model::event::Event;

pub type ArcComponent = Arc<dyn Component>;

/// Retour d'une commande
pub enum CommandMatch {
    /// La commande a été trouvée et traité
    Matched,
    /// La commande n'a pas été trouvée
    NotMatched,
    /// La commande a été trouvée mais une erreur s'est produite
    Error(String)
}

/// Trait de base des composants
/// 
/// Les composants doivent implémenter cette interface pour être utilisés par le framework et l'event dispatcher.
#[async_trait]
pub trait Component: Sync + Send 
{
    /// Nom du composant
    fn name(&self) -> &str;
    /// Command handler du composant.
    /// 
    /// Cette fonction est appelée lorsque le bot reçoie une commande.
    /// Elle doit retourner un [`CommandMatch`] qui définit si la commande a été traitée ou non.
    /// 
    /// Voir [`CommandMatch`], [`Context`] et [`Message`] pour plus d'informations.
    /// 
    /// [`Context`]: serenity::client::Context
    /// [`Message`]: serenity::model::channel::Message
    #[deprecated(note="Utilisez les events pour prendre en charge les slashs commandes")]
    async fn command(&self, _: &FrameworkConfig, _: &Context, _: &Message) -> CommandMatch {
        CommandMatch::NotMatched
    }
    /// Event handler du composant.
    /// 
    /// Cette fonction est appelée lorsque le bot reçoit un évènement.
    /// 
    /// Si l'event s'est bien passé ou n'a pas été traité, elle doit retourner `Ok(())`.
    /// Sinon, un Err contenant le message d'erreur doit être retourné. Ce message d'erreur sera ensuite renvoyé à la sortie standard.
    async fn event(&self, ctx: &Context, evt: &Event) -> Result<(), String>;
    /// Retournes le groupe de commandes lié au composant.
    /// 
    /// Le système d'aide du bot se repose sur ce groupe. 
    /// Vu que le parse de la commande n'est pas obligatoire, cette fonction est donc optionnelle.
    fn node(&self) -> Option<&command_parser::Node> {
        None
    }
    fn register_slash(&self, ) {
        
    }
    
}

pub trait ComponentExt: Sized {
    fn to_arc(self) -> Arc<Self> 
    {
        Arc::new(self)
    }
}
impl<T: Component> ComponentExt for T{
} 

