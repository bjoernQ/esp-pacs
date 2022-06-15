#[doc = "Register `LOG_SETTING` reader"]
pub struct R(crate::R<LOG_SETTING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOG_SETTING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOG_SETTING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOG_SETTING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOG_SETTING` writer"]
pub struct W(crate::W<LOG_SETTING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOG_SETTING_SPEC>;
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
impl From<crate::W<LOG_SETTING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOG_SETTING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOG_ENA` reader - reg_log_ena"]
pub type LOG_ENA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOG_ENA` writer - reg_log_ena"]
pub type LOG_ENA_W<'a> = crate::FieldWriter<'a, u32, LOG_SETTING_SPEC, u8, u8, 3, 0>;
#[doc = "Field `LOG_MODE` reader - reg_log_mode"]
pub type LOG_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOG_MODE` writer - reg_log_mode"]
pub type LOG_MODE_W<'a> = crate::FieldWriter<'a, u32, LOG_SETTING_SPEC, u8, u8, 4, 3>;
#[doc = "Field `LOG_MEM_LOOP_ENABLE` reader - reg_log_mem_loop_enable"]
pub type LOG_MEM_LOOP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `LOG_MEM_LOOP_ENABLE` writer - reg_log_mem_loop_enable"]
pub type LOG_MEM_LOOP_ENABLE_W<'a> = crate::BitWriter<'a, u32, LOG_SETTING_SPEC, bool, 7>;
impl R {
    #[doc = "Bits 0:2 - reg_log_ena"]
    #[inline(always)]
    pub fn log_ena(&self) -> LOG_ENA_R {
        LOG_ENA_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - reg_log_mode"]
    #[inline(always)]
    pub fn log_mode(&self) -> LOG_MODE_R {
        LOG_MODE_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - reg_log_mem_loop_enable"]
    #[inline(always)]
    pub fn log_mem_loop_enable(&self) -> LOG_MEM_LOOP_ENABLE_R {
        LOG_MEM_LOOP_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - reg_log_ena"]
    #[inline(always)]
    pub fn log_ena(&mut self) -> LOG_ENA_W {
        LOG_ENA_W::new(self)
    }
    #[doc = "Bits 3:6 - reg_log_mode"]
    #[inline(always)]
    pub fn log_mode(&mut self) -> LOG_MODE_W {
        LOG_MODE_W::new(self)
    }
    #[doc = "Bit 7 - reg_log_mem_loop_enable"]
    #[inline(always)]
    pub fn log_mem_loop_enable(&mut self) -> LOG_MEM_LOOP_ENABLE_W {
        LOG_MEM_LOOP_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ASSIST_DEBUG_LOG_SETTING\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [log_setting](index.html) module"]
pub struct LOG_SETTING_SPEC;
impl crate::RegisterSpec for LOG_SETTING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [log_setting::R](R) reader structure"]
impl crate::Readable for LOG_SETTING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [log_setting::W](W) writer structure"]
impl crate::Writable for LOG_SETTING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOG_SETTING to value 0x80"]
impl crate::Resettable for LOG_SETTING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
