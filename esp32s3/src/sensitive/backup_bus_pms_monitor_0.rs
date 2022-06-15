#[doc = "Register `BACKUP_BUS_PMS_MONITOR_0` reader"]
pub struct R(crate::R<BACKUP_BUS_PMS_MONITOR_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKUP_BUS_PMS_MONITOR_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKUP_BUS_PMS_MONITOR_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKUP_BUS_PMS_MONITOR_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BACKUP_BUS_PMS_MONITOR_0` writer"]
pub struct W(crate::W<BACKUP_BUS_PMS_MONITOR_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BACKUP_BUS_PMS_MONITOR_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BACKUP_BUS_PMS_MONITOR_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BACKUP_BUS_PMS_MONITOR_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BACKUP_BUS_PMS_MONITOR_LOCK` reader - Set 1 to lock BackUp permission report registers."]
pub type BACKUP_BUS_PMS_MONITOR_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `BACKUP_BUS_PMS_MONITOR_LOCK` writer - Set 1 to lock BackUp permission report registers."]
pub type BACKUP_BUS_PMS_MONITOR_LOCK_W<'a> =
    crate::BitWriter<'a, u32, BACKUP_BUS_PMS_MONITOR_0_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Set 1 to lock BackUp permission report registers."]
    #[inline(always)]
    pub fn backup_bus_pms_monitor_lock(&self) -> BACKUP_BUS_PMS_MONITOR_LOCK_R {
        BACKUP_BUS_PMS_MONITOR_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to lock BackUp permission report registers."]
    #[inline(always)]
    pub fn backup_bus_pms_monitor_lock(&mut self) -> BACKUP_BUS_PMS_MONITOR_LOCK_W {
        BACKUP_BUS_PMS_MONITOR_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BackUp permission report register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backup_bus_pms_monitor_0](index.html) module"]
pub struct BACKUP_BUS_PMS_MONITOR_0_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_MONITOR_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backup_bus_pms_monitor_0::R](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_MONITOR_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [backup_bus_pms_monitor_0::W](W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_MONITOR_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_MONITOR_0 to value 0"]
impl crate::Resettable for BACKUP_BUS_PMS_MONITOR_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
