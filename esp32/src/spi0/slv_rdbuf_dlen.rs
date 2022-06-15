#[doc = "Register `SLV_RDBUF_DLEN` reader"]
pub struct R(crate::R<SLV_RDBUF_DLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLV_RDBUF_DLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLV_RDBUF_DLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLV_RDBUF_DLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLV_RDBUF_DLEN` writer"]
pub struct W(crate::W<SLV_RDBUF_DLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLV_RDBUF_DLEN_SPEC>;
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
impl From<crate::W<SLV_RDBUF_DLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLV_RDBUF_DLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_RDBUF_DBITLEN` reader - In the slave mode it is the length in bits for read-buffer operations. The register value shall be (bit_num-1)."]
pub type SLV_RDBUF_DBITLEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SLV_RDBUF_DBITLEN` writer - In the slave mode it is the length in bits for read-buffer operations. The register value shall be (bit_num-1)."]
pub type SLV_RDBUF_DBITLEN_W<'a> =
    crate::FieldWriter<'a, u32, SLV_RDBUF_DLEN_SPEC, u32, u32, 24, 0>;
impl R {
    #[doc = "Bits 0:23 - In the slave mode it is the length in bits for read-buffer operations. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn slv_rdbuf_dbitlen(&self) -> SLV_RDBUF_DBITLEN_R {
        SLV_RDBUF_DBITLEN_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - In the slave mode it is the length in bits for read-buffer operations. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn slv_rdbuf_dbitlen(&mut self) -> SLV_RDBUF_DBITLEN_W {
        SLV_RDBUF_DBITLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slv_rdbuf_dlen](index.html) module"]
pub struct SLV_RDBUF_DLEN_SPEC;
impl crate::RegisterSpec for SLV_RDBUF_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slv_rdbuf_dlen::R](R) reader structure"]
impl crate::Readable for SLV_RDBUF_DLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slv_rdbuf_dlen::W](W) writer structure"]
impl crate::Writable for SLV_RDBUF_DLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLV_RDBUF_DLEN to value 0"]
impl crate::Resettable for SLV_RDBUF_DLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
